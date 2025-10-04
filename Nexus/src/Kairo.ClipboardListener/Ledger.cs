using System;
using System.IO;
using System.Text;
using System.Text.Json;

namespace Kairo.ClipboardListener
{
    public static class Ledger
    {
        public sealed class Record
        {
            public string Ts { get; set; } = string.Empty;
            public string Kind { get; set; } = string.Empty;
            public string File { get; set; } = string.Empty;
            public string? Sha256 { get; set; }
            public string Proto => "aitcp-hilr";
        }

        public static void Append(string ledgerDir, Record record)
        {
            Directory.CreateDirectory(ledgerDir);
            var day = DateTime.UtcNow.ToString("yyyyMMdd");
            var path = Path.Combine(ledgerDir, $"ledger-{day}.jsonl");
            var json = JsonSerializer.Serialize(record, new JsonSerializerOptions
            {
                Encoder = System.Text.Encodings.Web.JavaScriptEncoder.UnsafeRelaxedJsonEscaping
            });
            File.AppendAllText(path, json + Environment.NewLine, new UTF8Encoding(false));
        }
    }
}
