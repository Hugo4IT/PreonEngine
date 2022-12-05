using System.Runtime.InteropServices;

namespace Preon.Types;

[StructLayout(LayoutKind.Sequential)]
public struct PreonBorder
{
    public int top, right, bottom, left;

    public PreonBorder(int top, int right, int bottom, int left)
    {
        this.top = top;
        this.right = right;
        this.bottom = bottom;
        this.left = left;
    }

    public PreonBorder(int vertical, int horizontal)
    {
        this.top = vertical;
        this.right = horizontal;
        this.bottom = vertical;
        this.left = horizontal;
    }

    public PreonBorder(int width)
    {
        this.top = width;
        this.right = width;
        this.bottom = width;
        this.left = width;
    }
}