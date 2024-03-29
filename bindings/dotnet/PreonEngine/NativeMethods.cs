
using System.Runtime.InteropServices;
using System.Text;

using Preon.Rendering;
using Preon.Events;
using Preon.Types;

namespace Preon;

internal static class NativeMethods
{
    public unsafe delegate bool RunCallback(void* tree, PreonEventBinding two, PreonUserEventEmitterBinding three);

    [StructLayout(LayoutKind.Sequential)]
    public struct PreonEventBinding
    {
        internal byte Kind;

        internal PreonButtonState ButtonState;

        internal uint WindowResized_NewSize_X;
        internal uint WindowResized_NewSize_Y;

        internal string ComponentPressed_Id;

        internal ushort MouseInput_Button;
        internal PreonKeyCode KeyboardInput_Key;

        internal char ReceivedCharacter_Char;
    }

    // public static unsafe PreonEventBinding Bind(PreonEvent @event)
    // {
    //     return @event switch
    //     {
    //         PreonEvent.WindowOpened realEvent => new PreonEventBinding() { Kind = 1 },
    //         PreonEvent.WindowResized realEvent => new PreonEventBinding() { Kind = 0, WindowResized_NewSize_X = realEvent.NewSize.X, WindowResized_NewSize_Y = realEvent.NewSize.Y },
    //         PreonEvent.WindowClosed realEvent => new PreonEventBinding() { Kind = 2 },
    //         PreonEvent.Update realEvent => new PreonEventBinding() { Kind = 3 },
    //         PreonEvent.LayoutUpdate realEvent => new PreonEventBinding() { Kind = 4 },
    //         PreonEvent.Button realEvent => new PreonEventBinding() { Kind = 5, Button_Id = realEvent.Id, Button_State = realEvent.State },
    //         _ => throw new Exception("Nonexistant event kind"),
    //     };
    // }

    public static unsafe PreonEvent Unbind(PreonEventBinding binding)
    {
        return binding.Kind switch
        {
            0 => new PreonEvent.WindowOpened(),
            1 => new PreonEvent.WindowResized() { NewSize = new(binding.WindowResized_NewSize_X, binding.WindowResized_NewSize_Y) },
            2 => new PreonEvent.WindowClosed(),
            3 => new PreonEvent.Update(),
            4 => new PreonEvent.LayoutUpdate(),
            5 => new PreonEvent.ComponentPressed() { Id = binding.ComponentPressed_Id, State = binding.ButtonState },
            6 => new PreonEvent.MouseInput() { Index = binding.MouseInput_Button, State = binding.ButtonState },
            7 => new PreonEvent.KeyboardInput() { Key = binding.KeyboardInput_Key, State = binding.ButtonState },
            8 => new PreonEvent.ReceivedCharacter() { Char = binding.ReceivedCharacter_Char },
            byte other => throw new Exception($"Nonexistant event kind: {other}"),
        };
    }
    
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct PreonUserEventEmitterBinding
    {
        internal void* inner;
    }

    
    [DllImport("PreonEngine", EntryPoint = "PreonEngine__new", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void* PreonEngine__new();
    
    [DllImport("PreonEngine", EntryPoint = "PreonEngine__set_tree", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonEngine__set_tree(void* _0, void* _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__new", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void* PreonComponentBuilder__new();
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__id_string", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__id_string(void* _0, string _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__receive_events", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__receive_events(void* _0, bool _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__end", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__end(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__build", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void* PreonComponentBuilder__build(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__start_hbox", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_hbox(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__empty_hbox", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__empty_hbox(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__start_vbox", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_vbox(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__empty_vbox", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__empty_vbox(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__start_label", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_label(void* _0, string _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__empty_label", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__empty_label(void* _0, string _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__start_button", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_button(void* _0, string _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__empty_button", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__empty_button(void* _0, string _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__start_panel", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_panel(void* _0, PreonColor _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__empty_panel", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__empty_panel(void* _0, PreonColor _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__panel_color", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__panel_color(void* _0, PreonColor _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__start_static_texture", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__start_static_texture(void* _0, PreonImage _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__background_image", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__background_image(void* _0, PreonImage _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__background_color", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__background_color(void* _0, PreonColor _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__foreground_color", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__foreground_color(void* _0, PreonColor _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__align_items", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__align_items(void* _0, PreonAlignment _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__cross_align_items", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__cross_align_items(void* _0, PreonAlignment _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__layout", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__layout(void* _0, PreonLayout _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__margin", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__margin(void* _0, PreonBorder _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__padding", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__padding(void* _0, PreonBorder _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__border", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__border(void* _0, PreonBorder _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__corner_radius", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__corner_radius(void* _0, PreonCorners _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__min_size", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__min_size(void* _0, PreonVector<int> _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__fit_children", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__fit_children(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__fit_children_horizontally", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__fit_children_horizontally(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__fit_children_vertically", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__fit_children_vertically(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__expand", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__expand(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__expand_horizontally", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__expand_horizontally(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__expand_vertically", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__expand_vertically(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__text_vertical_align", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__text_vertical_align(void* _0, PreonAlignment _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__text_horizontal_align", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__text_horizontal_align(void* _0, PreonAlignment _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__font", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__font(void* _0, PreonFont _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponentBuilder__font_size", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponentBuilder__font_size(void* _0, float _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonEventEmitter__push", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonEventEmitter__push(void* _0, PreonEventBinding _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponent__set_text", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponent__set_text(void* _0, string _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponent__get_text", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern string PreonComponent__get_text(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponent__new", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void* PreonComponent__new();
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponent__add_child", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponent__add_child(void* _0, void* _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponent__insert_child", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponent__insert_child(void* _0, ushort _1, void* _2);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponent__remove_child", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponent__remove_child(void* _0, ushort _1);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponent__clear_children", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void PreonComponent__clear_children(void* _0);
    
    [DllImport("PreonEngine", EntryPoint = "PreonComponent__get_child_ref_mut_by_id", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void* PreonComponent__get_child_ref_mut_by_id(void* _0, string _1);
    
    [DllImport("PreonEngine", EntryPoint = "preon__init", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void preon__init();
    
    [DllImport("PreonEngine", EntryPoint = "preon__run", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void preon__run(void* _0, [MarshalAs(UnmanagedType.FunctionPtr)]RunCallback _1);
    
}
