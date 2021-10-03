using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading;

namespace ExoTracker {
	internal class Program {
		[DllImport("kernel32.dll")]
		private static extern IntPtr OpenProcess(int dwDesiredAccess, bool bInheritHandle, int dwProcessId);

		[DllImport("kernel32.dll")]
		private static extern bool ReadProcessMemory(IntPtr hProcess, ulong lpBaseAddress, byte[] lpBuffer, int dwSize, ref int lpNumberOfBytesRead);

		private static readonly int PROCESS_WM_READ = 0x0010;

		private static IntPtr processHandle;

		public static void Main(string[] args) {
			Console.Title = "ExoTracker v0.06";
			Foo();
		}

		static void Foo() {
			while (true) {
				try {
					Console.WriteLine("searching for game process...");
					Process? process;
					while (true) {
						process = Process.GetProcessesByName("EXO ONE").FirstOrDefault();
						if (process is {HasExited: false}) break;

						Thread.Sleep(1000);
					}
					Console.Clear();
					processHandle = OpenProcess(PROCESS_WM_READ, false, process.Id);
					var unityModule = process.Modules.Cast<ProcessModule>().First(module => module.FileName.EndsWith("UnityPlayer.dll"));

					while (true) {
						if (process.HasExited) {
							Console.Clear();
							break;
						}

						var address = ResolvePointer(unityModule, 0x0156C900, new uint[] { 0x3F8, 0x1A8, 0x28, 0xA0 });

						var positionX = ReadFloat(address + 0x00);
						var positionY = ReadFloat(address + 0x04);
						var positionZ = ReadFloat(address + 0x08);

						var distanceX = positionX - -66000f;
						var distanceZ = positionZ - 0f;
						var distanceTotal = Math.Sqrt(distanceX * distanceX + distanceZ * distanceZ);

						var velocityX = ReadFloat(address + 0x30);
						var velocityY = ReadFloat(address + 0x34);
						var velocityZ = ReadFloat(address + 0x38);

						var velocityHorizontal = Math.Sqrt(velocityX * velocityX + velocityZ * velocityZ);
						var velocityTotal = Math.Sqrt(velocityHorizontal * velocityHorizontal + velocityY * velocityY);

						Console.CursorLeft = 0;
						Console.CursorTop = 0;

						Console.WriteLine("velocity verti " + ((int)velocityY).ToString().PadRight(10));
						Console.WriteLine("velocity horiz " + ((int)velocityHorizontal).ToString().PadRight(10));
						Console.WriteLine("velocity total " + ((int)velocityTotal).ToString().PadRight(10));
						Console.WriteLine();
						Console.WriteLine("distance to go " + ((int)distanceTotal - 7500).ToString().PadRight(10));
						Console.WriteLine();

						Thread.Sleep(50);
					}
				}
				catch (Exception exception) {
					Console.WriteLine("ExoTracker crashed due to " + exception);
					Console.WriteLine("restarting...");
				}
			}
		}

		static ulong ResolvePointer(ProcessModule module, uint baseOffset, uint[] offsets) {
			var address = (ulong)module.BaseAddress + baseOffset;
			foreach (var offset in offsets) address = ReadULong(address) + offset;
			return address;
		}

		static byte[] ReadBytes(ulong addr, int count) {
			var buffer = new byte[count];
			var bytesRead = 0;
			ReadProcessMemory(processHandle, addr, buffer, count, ref bytesRead);
			return buffer;
		}

		static ulong ReadULong(ulong addr) => BitConverter.ToUInt64(ReadBytes(addr, 8), 0);
		static float ReadFloat(ulong addr) => BitConverter.ToSingle(ReadBytes(addr, 4), 0);
	}
}