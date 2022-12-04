using Preon.Rendering;
using Preon.Types;

namespace Preon;

public class PreonComponentBuilder
{
    private unsafe void* _inner;
    public PreonEngine EngineRef;

    public PreonComponentBuilder(PreonEngine engineRef)
    {
        EngineRef = engineRef;

        unsafe
        {
            _inner = NativeMethods.PreonComponentBuilder__new();
        }
    }

    public PreonComponentBuilder StartHBox()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__start_hbox(_inner);
        }

        return this;
    }

    public PreonComponentBuilder EmptyHBox()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__empty_hbox(_inner);
        }

        return this;
    }

    public PreonComponentBuilder StartVBox()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__start_vbox(_inner);
        }

        return this;
    }

    public PreonComponentBuilder EmptyVBox()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__empty_vbox(_inner);
        }

        return this;
    }

    public PreonComponentBuilder StartLabel(string text)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__start_label(_inner, text);
        }

        return this;
    }

    public PreonComponentBuilder EmptyLabel(string text)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__start_label(_inner, text);
        }

        return this;
    }

    public PreonComponentBuilder StartButton(string text)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__start_button(_inner, text);
        }

        return this;
    }

    public PreonComponentBuilder EmptyButton(string text)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__empty_button(_inner, text);
        }

        return this;
    }

    public PreonComponentBuilder StartPanel(PreonColor color)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__start_panel(_inner, color._inner);
        }

        return this;
    }

    public PreonComponentBuilder EmptyPanel(PreonColor color)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__empty_panel(_inner, color._inner);
        }

        return this;
    }

    public PreonComponentBuilder PanelColor(PreonColor color)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__panel_color(_inner, color._inner);
        }

        return this;
    }

    public PreonComponentBuilder StartStaticTexture(PreonImage image)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__start_static_texture(_inner, image._inner);
        }

        return this;
    }

    public PreonComponentBuilder BackgroundImage(PreonImage image)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__background_image(_inner, image._inner);
        }

        return this;
    }

    public PreonComponentBuilder BackgroundColor(PreonColor color)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__background_color(_inner, color._inner);
        }

        return this;
    }

    public PreonComponentBuilder Background(PreonImage image) => BackgroundImage(image);
    public PreonComponentBuilder Background(PreonColor color) => BackgroundColor(color);

    public PreonComponentBuilder ForegroundColor(PreonColor color)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__foreground_color(_inner, color._inner);
        }

        return this;
    }

    public PreonComponentBuilder AlignItems(PreonAlignment align)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__align_items(_inner, align);
        }

        return this;
    }

    public PreonComponentBuilder CrossAlignItems(PreonAlignment align)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__cross_align_items(_inner, align);
        }

        return this;
    }

    public PreonComponentBuilder Layout(PreonLayout layout)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__layout(_inner, layout);
        }

        return this;
    }

    public PreonComponentBuilder Margin(PreonBorder margin)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__margin(_inner, margin._inner);
        }

        return this;
    }

    public PreonComponentBuilder Padding(PreonBorder padding)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__padding(_inner, padding._inner);
        }

        return this;
    }

    public PreonComponentBuilder Border(PreonBorder border)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__border(_inner, border._inner);
        }

        return this;
    }

    public PreonComponentBuilder CornerRadius(PreonCorners corners)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__corner_radius(_inner, corners._inner);
        }

        return this;
    }

    public PreonComponentBuilder MinSize(PreonVector<int> minSize)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__min_size(_inner, minSize._inner);
        }

        return this;
    }

    public PreonComponentBuilder FitChildren()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__fit_children(_inner);
        }

        return this;
    }

    public PreonComponentBuilder FitChildrenHorizontally()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__fit_children_horizontally(_inner);
        }

        return this;
    }

    public PreonComponentBuilder FitChildrenVertically()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__fit_children_vertically(_inner);
        }

        return this;
    }

    public PreonComponentBuilder Expand()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__expand(_inner);
        }

        return this;
    }

    public PreonComponentBuilder ExpandHorizontally()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__expand_horizontally(_inner);
        }

        return this;
    }

    public PreonComponentBuilder ExpandVertically()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__expand_vertically(_inner);
        }

        return this;
    }

    public PreonComponentBuilder TextHorizontalAlign(PreonAlignment align)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__text_horizontal_align(_inner, align);
        }

        return this;
    }

    public PreonComponentBuilder TextVerticalAlign(PreonAlignment align)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__text_vertical_align(_inner, align);
        }

        return this;
    }
    
    public PreonComponentBuilder Font(PreonFont font)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__font(_inner, font._inner);
        }

        return this;
    }

    public PreonComponentBuilder FontSize(float size)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__font_size(_inner, size);
        }

        return this;
    }

    public PreonComponentBuilder ReceiveEvents(bool receiveEvents)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__receive_events(_inner, receiveEvents);
        }

        return this;
    }

    public PreonComponentBuilder Id(string id)
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__id_string(_inner, id);
        }

        return this;
    }

    public PreonComponentBuilder End()
    {
        unsafe
        {
            NativeMethods.PreonComponentBuilder__end(_inner);
        }

        return this;
    }

    public PreonComponent Build()
    {
        unsafe
        {
            return new PreonComponent(NativeMethods.PreonComponentBuilder__build(_inner));
        }
    }
}