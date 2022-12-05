using System.Runtime.InteropServices;

namespace Preon.Rendering;

[StructLayout(LayoutKind.Sequential)]
public struct PreonImage
{
    public unsafe void* imageRef;
    public uint padding; // 4 bytes padding
}