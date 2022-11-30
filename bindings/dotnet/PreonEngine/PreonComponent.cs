namespace Preon;

public class PreonComponent
{
    public string Text
    {
        get { unsafe { return NativeMethods.PreonComponent__get_text(_inner); } }
        set { unsafe { NativeMethods.PreonComponent__set_text(_inner, value); } }
    }

    internal unsafe void* _inner;

    internal unsafe PreonComponent(void* ptr)
    {
        _inner = ptr;
    }

    public PreonComponent GetChildById(string id)
    {
        unsafe
        {
            return new PreonComponent(NativeMethods.PreonComponent__get_child_ref_mut_by_id(_inner, id));
        }
    }

    public static PreonComponentBuilder StartBuilder()
    {
        return new PreonComponentBuilder();
    }

    ~PreonComponent()
    {
        Console.WriteLine("Destructor");
    }
}