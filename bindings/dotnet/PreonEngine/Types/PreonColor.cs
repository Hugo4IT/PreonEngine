using System.Runtime.InteropServices;

namespace Preon.Types;

[StructLayout(LayoutKind.Sequential)]
public struct PreonColor
{
    public float r, g, b, a;

    public static PreonColor White => PreonColor.Rgba32(1.0f, 1.0f, 1.0f, 1.0f);
    public static PreonColor TransparentWhite => PreonColor.Rgba32(1.0f, 1.0f, 1.0f, 0.0f);
    public static PreonColor Black => PreonColor.Rgba32(0.0f, 0.0f, 0.0f, 1.0f);
    public static PreonColor TransparentBlack => PreonColor.Rgba32(0.0f, 0.0f, 0.0f, 0.0f);

    public static PreonColor Light(float amount) => PreonColor.Rgba32(1.0f, 1.0f, 1.0f, amount);
    public static PreonColor Dark(float amount) => PreonColor.Rgba32(0.0f, 0.0f, 0.0f, amount);

    public static PreonColor Rgba32(float r, float g, float b, float a)
    {
        PreonColor color = new();

        color.r = r;
        color.g = g;
        color.b = b;
        color.a = a;

        return color;
    }

    public static PreonColor Rgb32(float r, float g, float b)
    {
        PreonColor color = new();
        
        color.r = r;
        color.g = g;
        color.b = b;
        color.a = 1.0f;

        return color;
    }

    public static PreonColor Rgba8(byte r, byte g, byte b, byte a)
    {
        PreonColor color = new();
        
        color.r = MathF.Pow(((float)r) / 255.0f, 2.2f);
        color.g = MathF.Pow(((float)g) / 255.0f, 2.2f);
        color.b = MathF.Pow(((float)b) / 255.0f, 2.2f);
        color.a = ((float)a) / 255.0f;

        return color;
    }

    public static PreonColor Rgb8(byte r, byte g, byte b)
    {
        PreonColor color = new();
        
        color.r = MathF.Pow(((float)r) / 255.0f, 2.2f);
        color.g = MathF.Pow(((float)g) / 255.0f, 2.2f);
        color.b = MathF.Pow(((float)b) / 255.0f, 2.2f);
        color.a = 1.0f;

        return color;
    }

    public static PreonColor? Hex(string hex)
    {
        string cleaned = hex.Replace("#", "").Replace("0x", "");
        if (cleaned.Length <= 4)
        {
            return PreonColor.Rgba8(
                Convert.ToByte(new string(cleaned[0], 2), 16),
                Convert.ToByte(new string(cleaned[1], 2), 16),
                Convert.ToByte(new string(cleaned[2], 2), 16),
                cleaned.Length == 4
                    ? Convert.ToByte(new string(cleaned[3], 2), 16)
                    : (byte)255
            );
        }
        else if (cleaned.Length == 6 || cleaned.Length == 8)
        {
            return PreonColor.Rgba8(
                Convert.ToByte(cleaned.Substring(0, 2), 16),
                Convert.ToByte(cleaned.Substring(2, 2), 16),
                Convert.ToByte(cleaned.Substring(4, 2), 16),
                cleaned.Length == 4
                    ? Convert.ToByte(cleaned.Substring(6, 2), 16)
                    : (byte)255
            );
        }

        return null;
    }
}