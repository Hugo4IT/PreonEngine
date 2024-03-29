import os
import os.path

PTR = "void*"
BINDINGS = [
    [PTR,    "PreonEngine__new",      []],
    ["void", "PreonEngine__set_tree", [PTR, PTR]],


    [PTR,    "PreonComponentBuilder__new",            []],
    ["void", "PreonComponentBuilder__id_string",      [PTR, "string"]],
    ["void", "PreonComponentBuilder__receive_events", [PTR, "bool"]],
    ["void", "PreonComponentBuilder__end",            [PTR]],
    [PTR,    "PreonComponentBuilder__build",          [PTR]],

    ["void",    "PreonComponentBuilder__start_hbox", [PTR]],
    ["void",    "PreonComponentBuilder__empty_hbox", [PTR]],

    ["void",    "PreonComponentBuilder__start_vbox", [PTR]],
    ["void",    "PreonComponentBuilder__empty_vbox", [PTR]],

    ["void",    "PreonComponentBuilder__start_label", [PTR, "string"]],
    ["void",    "PreonComponentBuilder__empty_label", [PTR, "string"]],

    ["void",    "PreonComponentBuilder__start_button", [PTR, "string"]],
    ["void",    "PreonComponentBuilder__empty_button", [PTR, "string"]],

    ["void",    "PreonComponentBuilder__start_panel", [PTR, "PreonColor"]],
    ["void",    "PreonComponentBuilder__empty_panel", [PTR, "PreonColor"]],
    ["void",    "PreonComponentBuilder__panel_color", [PTR, "PreonColor"]],

    ["void",    "PreonComponentBuilder__start_static_texture", [PTR, "PreonImage"]],

    ["void",    "PreonComponentBuilder__background_image",          [PTR, "PreonImage"]],
    ["void",    "PreonComponentBuilder__background_color",          [PTR, "PreonColor"]],
    ["void",    "PreonComponentBuilder__foreground_color",          [PTR, "PreonColor"]],
    ["void",    "PreonComponentBuilder__align_items",               [PTR, "PreonAlignment"]],
    ["void",    "PreonComponentBuilder__cross_align_items",         [PTR, "PreonAlignment"]],
    ["void",    "PreonComponentBuilder__layout",                    [PTR, "PreonLayout"]],
    ["void",    "PreonComponentBuilder__margin",                    [PTR, "PreonBorder"]],
    ["void",    "PreonComponentBuilder__padding",                   [PTR, "PreonBorder"]],
    ["void",    "PreonComponentBuilder__border",                    [PTR, "PreonBorder"]],
    ["void",    "PreonComponentBuilder__corner_radius",             [PTR, "PreonCorners"]],
    ["void",    "PreonComponentBuilder__min_size",                  [PTR, "PreonVector<int>"]],
    ["void",    "PreonComponentBuilder__fit_children",              [PTR]],
    ["void",    "PreonComponentBuilder__fit_children_horizontally", [PTR]],
    ["void",    "PreonComponentBuilder__fit_children_vertically",   [PTR]],
    ["void",    "PreonComponentBuilder__expand",                    [PTR]],
    ["void",    "PreonComponentBuilder__expand_horizontally",       [PTR]],
    ["void",    "PreonComponentBuilder__expand_vertically",         [PTR]],

    ["void",    "PreonComponentBuilder__text_vertical_align",   [PTR, "PreonAlignment"]],
    ["void",    "PreonComponentBuilder__text_horizontal_align", [PTR, "PreonAlignment"]],
    ["void",    "PreonComponentBuilder__font",                  [PTR, "PreonFont"]],
    ["void",    "PreonComponentBuilder__font_size",             [PTR, "float"]],

    ["void", "PreonEventEmitter__push", [PTR, "PreonEventBinding"]],

    ["void",       "PreonComponent__set_text",  [PTR, "string"]],
    ["string",     "PreonComponent__get_text",  [PTR]],

    [PTR,      "PreonComponent__new",            []],
    ["void",   "PreonComponent__add_child",      [PTR, PTR]],
    ["void",   "PreonComponent__insert_child",   [PTR, "ushort", PTR]],
    ["void",   "PreonComponent__remove_child",   [PTR, "ushort"]],
    ["void",   "PreonComponent__clear_children", [PTR]],

    [PTR,      "PreonComponent__get_child_ref_mut_by_id", [PTR, "string"]],

    ["void", "preon__init",           []],
    ["void", "preon__run",            [PTR, "[MarshalAs(UnmanagedType.FunctionPtr)]RunCallback"]],
]

methods = ""
for binding in BINDINGS:
    returntype, funcname, parametertypes = binding
    methods += \
    f"""
    [DllImport("PreonEngine", EntryPoint = "{funcname}", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern {returntype} {funcname}({', '.join([paramtype + " _" + str(i) for i, paramtype in enumerate(parametertypes)])});
    """


output = \
f"""
using System.Runtime.InteropServices;
using System.Text;

using Preon.Rendering;
using Preon.Events;
using Preon.Types;

namespace Preon;

internal static class NativeMethods
{{
    public unsafe delegate bool RunCallback({PTR} tree, PreonEventBinding two, PreonUserEventEmitterBinding three);

    [StructLayout(LayoutKind.Sequential)]
    public struct PreonEventBinding
    {{
        internal byte Kind;

        internal PreonButtonState ButtonState;

        internal uint WindowResized_NewSize_X;
        internal uint WindowResized_NewSize_Y;

        internal string ComponentPressed_Id;

        internal ushort MouseInput_Button;
        internal PreonKeyCode KeyboardInput_Key;

        internal char ReceivedCharacter_Char;
    }}

    // public static unsafe PreonEventBinding Bind(PreonEvent @event)
    // {{
    //     return @event switch
    //     {{
    //         PreonEvent.WindowOpened realEvent => new PreonEventBinding() {{ Kind = 1 }},
    //         PreonEvent.WindowResized realEvent => new PreonEventBinding() {{ Kind = 0, WindowResized_NewSize_X = realEvent.NewSize.X, WindowResized_NewSize_Y = realEvent.NewSize.Y }},
    //         PreonEvent.WindowClosed realEvent => new PreonEventBinding() {{ Kind = 2 }},
    //         PreonEvent.Update realEvent => new PreonEventBinding() {{ Kind = 3 }},
    //         PreonEvent.LayoutUpdate realEvent => new PreonEventBinding() {{ Kind = 4 }},
    //         PreonEvent.Button realEvent => new PreonEventBinding() {{ Kind = 5, Button_Id = realEvent.Id, Button_State = realEvent.State }},
    //         _ => throw new Exception("Nonexistant event kind"),
    //     }};
    // }}

    public static unsafe PreonEvent Unbind(PreonEventBinding binding)
    {{
        return binding.Kind switch
        {{
            0 => new PreonEvent.WindowOpened(),
            1 => new PreonEvent.WindowResized() {{ NewSize = new(binding.WindowResized_NewSize_X, binding.WindowResized_NewSize_Y) }},
            2 => new PreonEvent.WindowClosed(),
            3 => new PreonEvent.Update(),
            4 => new PreonEvent.LayoutUpdate(),
            5 => new PreonEvent.ComponentPressed() {{ Id = binding.ComponentPressed_Id, State = binding.ButtonState }},
            6 => new PreonEvent.MouseInput() {{ Index = binding.MouseInput_Button, State = binding.ButtonState }},
            7 => new PreonEvent.KeyboardInput() {{ Key = binding.KeyboardInput_Key, State = binding.ButtonState }},
            8 => new PreonEvent.ReceivedCharacter() {{ Char = binding.ReceivedCharacter_Char }},
            byte other => throw new Exception($"Nonexistant event kind: {{other}}"),
        }};
    }}
    
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct PreonUserEventEmitterBinding
    {{
        internal void* inner;
    }}

    {methods}
}}
"""


with open(os.path.join(os.path.dirname(__file__), "PreonEngine/NativeMethods.cs"), "w+") as file:
    file.write(output)