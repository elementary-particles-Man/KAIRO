using System;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Windows.Forms;

namespace Kairo.ClipboardListener
{
    public sealed class ClipboardWatcher : Form
    {
        private const int WmClipboardupdate = 0x031D;
        private const int MaxPayloadBytes = 512 * 1024;
        private static readonly TimeSpan ClipboardRetryDelay = TimeSpan.FromMilliseconds(40);
        private const int ClipboardRetryCount = 6;

        private readonly NotifyIcon _tray;
        private readonly ContextMenuStrip _menu;
        private readonly string _rootDir;
        private readonly string _inboxDir;
        private readonly string _ledgerDir;

        private string? _lastSha;
        private bool _isProcessing;
        private bool _pending;

        public ClipboardWatcher()
        {
            _rootDir = ResolveRoot();
            _inboxDir = Path.Combine(_rootDir, "inbox");
            _ledgerDir = Path.Combine(_rootDir, "ledger");

            Directory.CreateDirectory(_inboxDir);
            Directory.CreateDirectory(_ledgerDir);

            ShowInTaskbar = false;
            WindowState = FormWindowState.Minimized;
            FormBorderStyle = FormBorderStyle.FixedToolWindow;
            Opacity = 0;
            Size = new Size(0, 0);

            _menu = new ContextMenuStrip();
            _menu.Items.Add("Open Inbox", null, (_, _) => OpenInbox());
            _menu.Items.Add(new ToolStripSeparator());
            _menu.Items.Add("Exit", null, (_, _) => Application.Exit());

            _tray = new NotifyIcon
            {
                Icon = SystemIcons.Application,
                Visible = true,
                Text = "KAIRO Clipboard Listener",
                ContextMenuStrip = _menu
            };

            Shown += (_, _) => Hide();
            Application.ApplicationExit += (_, _) => CleanupTray();
        }

        protected override void OnHandleCreated(EventArgs e)
        {
            base.OnHandleCreated(e);
            try
            {
                NativeMethods.AddClipboardFormatListener(Handle);
            }
            catch
            {
                // listener registration failure would surface on first update; no-op here
            }
        }

        protected override void OnHandleDestroyed(EventArgs e)
        {
            try
            {
                NativeMethods.RemoveClipboardFormatListener(Handle);
            }
            catch
            {
                // ignore
            }
            base.OnHandleDestroyed(e);
        }

        protected override void OnFormClosed(FormClosedEventArgs e)
        {
            CleanupTray();
            base.OnFormClosed(e);
        }

        protected override void WndProc(ref Message m)
        {
            if (m.Msg == WmClipboardupdate)
            {
                HandleClipboardUpdate();
            }

            base.WndProc(ref m);
        }

        private void HandleClipboardUpdate()
        {
            if (_isProcessing)
            {
                _pending = true;
                return;
            }

            _isProcessing = true;
            BeginInvoke(new Action(ProcessClipboard));
        }

        private void ProcessClipboard()
        {
            try
            {
                var text = TryReadClipboardText();
                if (text == null)
                {
                    return;
                }

                text = text.Trim();
                if (text.Length == 0)
                {
                    return;
                }

                var utf8 = Encoding.UTF8.GetBytes(text);
                if (utf8.Length == 0 || utf8.Length > MaxPayloadBytes)
                {
                    return;
                }

                if (!IsTargetPayload(utf8))
                {
                    return;
                }

                var sha = Sha256Hex(utf8);
                if (sha == _lastSha)
                {
                    return;
                }

                var utcNow = DateTime.UtcNow;
                var fileName = $"capture-{utcNow:yyyyMMddTHHmmssfffZ}-{Guid.NewGuid():N}.json";
                var filePath = Path.Combine(_inboxDir, fileName);

                File.WriteAllText(filePath, text, new UTF8Encoding(false));

                try
                {
                    Ledger.Append(_ledgerDir, new Ledger.Record
                    {
                        Ts = utcNow.ToString("o"),
                        Kind = "capture",
                        File = fileName,
                        Sha256 = sha
                    });
                }
                catch
                {
                    // ledger failures should not stop capture; silently ignore
                }

                _lastSha = sha;
                ShowBalloon("Clipboard captured", fileName);
            }
            catch
            {
                // swallow transient clipboard issues
            }
            finally
            {
                _isProcessing = false;
                if (_pending)
                {
                    _pending = false;
                    HandleClipboardUpdate();
                }
            }
        }

        private string? TryReadClipboardText()
        {
            for (var attempt = 0; attempt < ClipboardRetryCount; attempt++)
            {
                try
                {
                    if (!Clipboard.ContainsText(TextDataFormat.UnicodeText))
                    {
                        return null;
                    }

                    var text = Clipboard.GetText(TextDataFormat.UnicodeText);
                    return string.IsNullOrEmpty(text) ? null : text;
                }
                catch (ExternalException)
                {
                    Thread.Sleep(ClipboardRetryDelay);
                }
                catch
                {
                    return null;
                }
            }

            return null;
        }

        private static bool IsTargetPayload(byte[] utf8)
        {
            try
            {
                var reader = new Utf8JsonReader(utf8, new JsonReaderOptions
                {
                    AllowTrailingCommas = true,
                    CommentHandling = JsonCommentHandling.Skip
                });

                if (!JsonDocument.TryParseValue(ref reader, out var doc))
                {
                    return false;
                }

                using (doc)
                {
                    if (doc.RootElement.ValueKind != JsonValueKind.Object)
                    {
                        return false;
                    }

                    if (!doc.RootElement.TryGetProperty("proto_ver", out var proto) || proto.ValueKind != JsonValueKind.String)
                    {
                        return false;
                    }

                    var value = proto.GetString();
                    return !string.IsNullOrEmpty(value) && value.StartsWith("aitcp-hilr-", StringComparison.Ordinal);
                }
            }
            catch (JsonException)
            {
                return false;
            }
        }

        private static string ResolveRoot()
        {
            var env = Environment.GetEnvironmentVariable("KAIRO_NEXUS_ROOT");
            if (!string.IsNullOrWhiteSpace(env))
            {
                try
                {
                    return Path.GetFullPath(env);
                }
                catch
                {
                    // fall back to default
                }
            }

            var local = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
            return Path.Combine(local, "KAIRO", "Nexus");
        }

        private void OpenInbox()
        {
            try
            {
                Process.Start(new ProcessStartInfo
                {
                    FileName = _inboxDir,
                    UseShellExecute = true
                });
            }
            catch
            {
                // ignore failures opening folder
            }
        }

        private void ShowBalloon(string title, string message)
        {
            try
            {
                _tray.BalloonTipTitle = title;
                _tray.BalloonTipText = message;
                _tray.BalloonTipIcon = ToolTipIcon.Info;
                _tray.ShowBalloonTip(2000);
            }
            catch
            {
                // ignore balloon errors
            }
        }

        private void CleanupTray()
        {
            _tray.Visible = false;
            _tray.Dispose();
            _menu.Dispose();
        }

        private static string Sha256Hex(byte[] data)
        {
            using var hash = SHA256.Create();
            var bytes = hash.ComputeHash(data);
            var sb = new StringBuilder(bytes.Length * 2);
            foreach (var b in bytes)
            {
                _ = sb.Append(b.ToString("x2"));
            }

            return sb.ToString();
        }

        private static class NativeMethods
        {
            [DllImport("user32.dll", SetLastError = true)]
            public static extern bool AddClipboardFormatListener(IntPtr hwnd);

            [DllImport("user32.dll", SetLastError = true)]
            public static extern bool RemoveClipboardFormatListener(IntPtr hwnd);
        }
    }
}
