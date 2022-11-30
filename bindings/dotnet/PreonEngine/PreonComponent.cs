namespace Preon;

public class PreonComponent
{
    public string Text
    {
        get { unsafe { return NativeMethods.Unbind(NativeMethods.PreonComponent__get_text(_inner)); } }
        set { unsafe { NativeMethods.PreonComponent__set_text(_inner, NativeMethods.Bind(value)); } }
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
            return new PreonComponent(NativeMethods.PreonComponent__get_child_ref_mut_by_id(_inner, NativeMethods.Bind(id)));
        }
    }

    public void Test()
    {
        unsafe
        {
            NativeMethods.PreonComponent__test(_inner, NativeMethods.Bind("This is a message from C#"));
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