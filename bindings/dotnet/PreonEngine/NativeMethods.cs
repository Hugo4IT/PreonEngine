
using System.Runtime.InteropServices;
using System.Text;

using Preon.Rendering;
using Preon.Events;
using Preon.Types;

namespace Preon;

internal static class NativeMethods
{
    public unsafe delegate void RunCallback(void* tree, PreonEventBinding two, PreonUserEventEmitterBinding three);

    [StructLayout(LayoutKind.Explicit)]
    public unsafe struct PreonEventBinding
    {
        [FieldOffset(0)]
        internal byte kind;

        [FieldOffset(1)]
        internal uint WindowResized_NewSize_X;
        [FieldOffset(5)]
        internal uint WindowResized_NewSize_Y;

        [FieldOffset(1)]
        internal uint Button_Id;
        [FieldOffset(5)]
        internal PreonButtonState Button_State;
    }

    public static unsafe PreonEventBinding Bind(PreonEvent @event)
    {
        return @event switch
        {
            PreonEvent.WindowResized realEvent => new PreonEventBinding() { kind = 0, WindowResized_NewSize_X = realEvent.NewSize.X, WindowResized_NewSize_Y = realEvent.NewSize.Y },
            PreonEvent.WindowOpened realEvent => new PreonEventBinding() { kind = 1 },
            PreonEvent.WindowClosed realEvent => new PreonEventBinding() { kind = 2 },
            PreonEvent.Update realEvent => new PreonEventBinding() { kind = 3 },
            PreonEvent.LayoutUpdate realEvent => new PreonEventBinding() { kind = 4 },
            PreonEvent.Button realEvent => new PreonEventBinding() { kind = 5, Button_Id = realEvent.Id, Button_State = realEvent.State },
            _ => throw new Exception("Nonexistant event kind"),
        };
    }

    public static unsafe PreonEvent Unbind(PreonEventBinding binding)
    {
        Console.WriteLine($"{binding.kind} {binding.WindowResized_NewSize_X} {binding.WindowResized_NewSize_Y} {binding.Button_State} {binding.Button_Id}");
        return binding.kind switch
        {
            0 => new PreonEvent.WindowResized() { NewSize = new(binding.WindowResized_NewSize_X, binding.WindowResized_NewSize_Y) },
            1 => new PreonEvent.WindowOpened(),
            2 => new PreonEvent.WindowClosed(),
            3 => new PreonEvent.Update(),
            4 => new PreonEvent.LayoutUpdate(),
            5 => new PreonEvent.Button() { Id = binding.Button_Id, State = binding.Button_State },
            _ => throw new Exception("Nonexistant event kind"),
        };
    }
    
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct PreonUserEventEmitterBinding
    {
        internal void* inner;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct StringBinding
    {
        internal nuint length;
        internal byte* ptr;
    }

    public static StringBinding Bind(string str)
    {
        unsafe
        {
            fixed(byte* ptr = Encoding.UTF8.GetBytes(str))
            {
                return new StringBinding()
                {
                    length = (nuint)str.Length,
                    ptr = ptr
                };
            }
        }
    }

    public static string Unbind(StringBinding str)
    {
        unsafe
        {
            return Marshal.PtrToStringUTF8(new IntPtr(str.ptr), (int)str.length);
        }
    }

    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonEngine__new", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void* PreonEngine__new();
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonEngine__set_tree", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonEngine__set_tree(void* _0, void* _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__new", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void* PreonComponentBuilder__new();
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__id_string", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__id_string(void* _0, StringBinding _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__end", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__end(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__build", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void* PreonComponentBuilder__build(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__start_hbox", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_hbox(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__empty_hbox", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__empty_hbox(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__start_vbox", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_vbox(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__empty_vbox", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__empty_vbox(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__start_label", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_label(void* _0, StringBinding _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__empty_label", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__empty_label(void* _0, StringBinding _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__start_panel", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_panel(void* _0, PreonColor.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__empty_panel", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__empty_panel(void* _0, PreonColor.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__panel_color", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__panel_color(void* _0, PreonColor.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__start_static_texture", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_static_texture(void* _0, PreonImage.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__background_image", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__background_image(void* _0, PreonImage.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__background_color", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__background_color(void* _0, PreonColor.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__foreground_color", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__foreground_color(void* _0, PreonColor.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__align_items", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__align_items(void* _0, PreonAlignment _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__cross_align_items", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__cross_align_items(void* _0, PreonAlignment _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__layout", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__layout(void* _0, PreonLayout _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__margin", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__margin(void* _0, PreonBorder.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__padding", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__padding(void* _0, PreonBorder.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__border", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__border(void* _0, PreonBorder.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__corner_radius", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__corner_radius(void* _0, PreonCorners.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__min_size", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__min_size(void* _0, PreonVector<int>.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__fit_children", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__fit_children(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__fit_children_horizontally", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__fit_children_horizontally(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__fit_children_vertically", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__fit_children_vertically(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__expand", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__expand(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__expand_horizontally", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__expand_horizontally(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__expand_vertically", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__expand_vertically(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__text_vertical_align", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__text_vertical_align(void* _0, PreonAlignment _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__text_horizontal_align", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__text_horizontal_align(void* _0, PreonAlignment _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__font", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__font(void* _0, PreonFont.Inner _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponentBuilder__font_size", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__font_size(void* _0, float _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonEventEmitter__push", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonEventEmitter__push(void* _0, PreonEventBinding _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponent__set_text", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponent__set_text(void* _0, StringBinding _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponent__get_text", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern StringBinding PreonComponent__get_text(void* _0);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponent__test", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern StringBinding PreonComponent__test(void* _0, StringBinding _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "PreonComponent__get_child_ref_mut_by_id", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void* PreonComponent__get_child_ref_mut_by_id(void* _0, StringBinding _1);
    
    [DllImport("PreonEngine.dll", EntryPoint = "preon__init", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void preon__init();
    
    [DllImport("PreonEngine.dll", EntryPoint = "preon__run", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void preon__run(void* _0, [MarshalAs(UnmanagedType.FunctionPtr)]RunCallback _1);
    
}
