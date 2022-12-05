using System.Runtime.InteropServices;

namespace Preon.Rendering;

[StructLayout(LayoutKind.Sequential)]
public struct PreonFont
{
    public unsafe void* fontRef;
    public uint padding; // 4 Bytes padding
}