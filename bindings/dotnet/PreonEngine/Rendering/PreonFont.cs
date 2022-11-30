using System.Runtime.InteropServices;

namespace Preon.Rendering;

public class PreonFont
{
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct Inner
    {
        void* fontRef;
    }

    internal Inner _inner;
}