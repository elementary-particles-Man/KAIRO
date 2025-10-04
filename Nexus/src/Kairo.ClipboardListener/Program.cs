using System;
using System.Threading;
using System.Windows.Forms;

namespace Kairo.ClipboardListener
{
    internal static class Program
    {
        private static Mutex? _mutex;

        [STAThread]
        private static void Main()
        {
            var createdNew = false;
            _mutex = new Mutex(true, "KAIRO_ClipboardListener_Singleton", out createdNew);
            if (!createdNew)
            {
                return;
            }

            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new ClipboardWatcher());
            _mutex.ReleaseMutex();
        }
    }
}
