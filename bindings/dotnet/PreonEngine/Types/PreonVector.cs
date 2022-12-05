using System.Runtime.InteropServices;

namespace Preon.Types;

[StructLayout(LayoutKind.Sequential)]
public struct PreonVector<T>
{
    public T x, y;

    public PreonVector(T x, T y)
    {
        this.x = x;
        this.y = y;
    }
}