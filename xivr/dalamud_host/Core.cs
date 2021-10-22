﻿using Dalamud.Plugin;
using Dalamud.Logging;
using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using ImGuiNET;

namespace XIVR
{
    static class NativeMethods
    {
        [DllImport("kernel32.dll", SetLastError = true)]
        public static extern IntPtr LoadLibrary(string dllToLoad);

        [DllImport("kernel32.dll", SetLastError = true)]
        public static extern IntPtr GetProcAddress(IntPtr hModule, string procedureName);

        [DllImport("kernel32.dll", SetLastError = true)]
        public static extern bool FreeLibrary(IntPtr hModule);
    }

    [UnmanagedFunctionPointer(CallingConvention.Winapi)]
    public delegate void LogDelegate(string s);

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)]
    public unsafe struct LoadParameters
    {
        public LogDelegate logger;
        public IntPtr imguiContext;
        public IntPtr imguiAllocatorAlloc;
        public IntPtr imguiAllocatorFree;
        public void* imguiAllocatorUserData;
    }

    public class Core : IDalamudPlugin
    {
        public string Name => "XIVR Core";

        private DalamudPluginInterface pi;
        private FileSystemWatcher watcher;

        // When loaded by LivePluginLoader, the executing assembly will be wrong.
        // Supplying this property allows LivePluginLoader to supply the correct location, so that
        // you have full compatibility when loaded normally and through LPL.
        public string AssemblyLocation { get => assemblyLocation; set => assemblyLocation = value; }
        private string assemblyLocation = System.Reflection.Assembly.GetExecutingAssembly().Location;

        private bool ReloadQueued = false;

        private string DirPath { get => Path.GetFullPath(Path.GetDirectoryName(assemblyLocation)!); }
        private string ModuleName(string ext) => "xivr_native" + "." + ext;
        private string ModulePath(string ext) => Path.Combine(DirPath, ModuleName(ext));
        private string ModuleLoadedName(string ext) => "xivr_native_loaded" + "." + ext;
        private string ModuleLoadedPath(string ext) => Path.Combine(DirPath, ModuleLoadedName(ext));
        private IntPtr module = IntPtr.Zero;
        private bool visible = true;

        private LogDelegate logDelegate = (s) => PluginLog.Information("native: {0:l}", s);

        public Core(DalamudPluginInterface pluginInterface)
        {
            this.pi = pluginInterface;

            this.pi.UiBuilder.Draw += this.OnDraw;
        }

        public void Dispose()
        {
            Unload(() => { });

            this.pi.Dispose();
        }

        [UnmanagedFunctionPointer(CallingConvention.Winapi)]
        delegate bool LoadType(IntPtr loadParams);

        [UnmanagedFunctionPointer(CallingConvention.Winapi)]
        delegate void UnloadType();

        [UnmanagedFunctionPointer(CallingConvention.Winapi)]
        delegate void DrawType();

        private TDelegate ModuleFunction<TDelegate>(string name)
        {
            var functionPointer = NativeMethods.GetProcAddress(this.module, name);
            return Marshal.GetDelegateForFunctionPointer<TDelegate>(functionPointer);
        }

        private void Reload()
        {
            PluginLog.Information("Reloading...");

            if (this.module != IntPtr.Zero)
            {
                // On unload, we resize the window. This causes the D3D device to be invalidated,
                // and we don't want to start up OpenXR with an invalid device.
                // Instead, let's use more jank to delay the startup until we can be sure we're good to go.
                this.Unload(() => Task.Delay(2500).ContinueWith(_ => this.Load()));
            }
            else
            {
                this.Load();
            }
        }

        private void DestroyModule()
        {
            if (this.module == IntPtr.Zero) return;

            NativeMethods.FreeLibrary(this.module);
            this.module = IntPtr.Zero;
        }

        private void Unload(Action onUnload)
        {
            if (this.module == IntPtr.Zero) return;

            ModuleFunction<UnloadType>("xivr_unload")();

            this.DestroyModule();
            onUnload();
        }

        private void Load()
        {
            if (this.module != IntPtr.Zero) return;
            try
            {
                File.Copy(ModulePath("dll"), ModuleLoadedPath("dll"), true);
                File.Copy(ModulePath("pdb"), ModuleLoadedPath("pdb"), true);

                this.module = NativeMethods.LoadLibrary(ModuleLoadedPath("dll"));
                if (this.module == IntPtr.Zero)
                {
                    throw new Exception(string.Format("Failed to load native module: {0}", Marshal.GetLastWin32Error()));
                }

                unsafe
                {
                    LoadParameters parameters = default;
                    parameters.logger = this.logDelegate;
                    parameters.imguiContext = ImGui.GetCurrentContext();
                    ImGui.GetAllocatorFunctions(
                        ref parameters.imguiAllocatorAlloc,
                        ref parameters.imguiAllocatorFree,
                        ref parameters.imguiAllocatorUserData
                    );

                    IntPtr ptr = Marshal.AllocHGlobal(Marshal.SizeOf(parameters));
                    Marshal.StructureToPtr(parameters, ptr, false);

                    try
                    {
                        ModuleFunction<LoadType>("xivr_load")(ptr);
                    }
                    finally
                    {
                        Marshal.FreeHGlobal(ptr);
                    }
                }
            }
            finally
            {
                this.ReloadQueued = false;
            }
        }

        private void OnDraw()
        {
            if (ImGui.Begin("XIVR Loader", ref this.visible, ImGuiWindowFlags.NoScrollbar | ImGuiWindowFlags.NoScrollWithMouse))
            {
                if (ImGui.Button("Reload"))
                {
                    Reload();
                }
                if (this.module == IntPtr.Zero)
                {
                    if (ImGui.Button("Load"))
                    {
                        Load();
                    }
                }
                else
                {
                    if (ImGui.Button("Unload"))
                    {
                        Unload(() => { });
                    }
                }
            }

            if (this.module == IntPtr.Zero) return;
            ModuleFunction<DrawType>("xivr_draw_ui")();
        }
    }
}