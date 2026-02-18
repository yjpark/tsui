use tsui_model::Model;
use tsui_viewmodel::ViewModel;

pub trait View {
    type Model: Model;
    type ViewModel: ViewModel;

    fn get_model(&self) -> &Self::Model;
    fn get_viewmodel(&self) -> &Self::ViewModel;
}
