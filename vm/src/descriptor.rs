use crate::function::OptionalArg;
use crate::pyobject::{IdProtocol, PyObjectRef, PyRef, PyResult, PyValue};
use crate::VirtualMachine;

#[pyimpl]
pub trait PyBuiltinDescriptor: PyValue {
    #[pymethod(name = "__get__")]
    #[pyslot(descr_get)]
    fn get(
        zelf: PyRef<Self>,
        obj: PyObjectRef,
        cls: OptionalArg<PyObjectRef>,
        vm: &VirtualMachine,
    ) -> PyResult;

    fn _cls_is<T>(cls: &OptionalArg<PyObjectRef>, other: &T) -> bool
    where
        T: IdProtocol,
    {
        match cls {
            OptionalArg::Present(cls) => cls.is(other),
            OptionalArg::Missing => false,
        }
    }
}
