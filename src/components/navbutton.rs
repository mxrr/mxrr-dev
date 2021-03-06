use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::AppRoutes;
use yewtil::NeqAssign;
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub to: AppRoutes,
  pub text: String,
  pub active: AppRoutes,
  pub styles: Option<String>,
}

pub struct NavButton {
  props: Props,
}

impl Component for NavButton {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
      props
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
  }

  fn view(&self) -> Html {
    let mut classlist = classes!("text-base-lt hover:text-base-dk hover:bg-base-lt rounded-l-md transition-all px-5 py-1".to_string());

    classlist.push(self.props.styles.clone());

    if self.props.active == self.props.to {
      classlist.push("bg-primary-accent-dk bg-opacity-50 border-l-4 border-secondary-accent-md");
    }  else {
      classlist.push("bg-primary-accent-dk bg-opacity-30 border-l-4 border-base-lt");
    }

    html! {
      <Link<AppRoutes> 
        route=self.props.to.clone()
        classes=classlist
      >
        { self.props.text.clone() }
      </Link<AppRoutes>>
    }
  }
}