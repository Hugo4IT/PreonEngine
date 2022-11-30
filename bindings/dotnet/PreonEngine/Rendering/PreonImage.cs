using System.Runtime.InteropServices;

namespace Preon.Rendering;

public class PreonImage
{
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct Inner
    {
        void* imageRef;
    }

    internal Inner _inner;
}