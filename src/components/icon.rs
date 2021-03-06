use yew::prelude::*;
use yewtil::NeqAssign;


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub icon: String,
  pub disable_invert: Option<bool>,
}

pub struct Icon {
  props: Props,
}

impl Component for Icon {
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
    let icon = match self.props.icon.as_str() {
      "gh" => "/static/GitHub-Mark-32px.png",
      "image" => "/static/image.png",
      "compass" => "/static/compass.png",
      "linked-in" => "/static/linked-in.png",
      "steam" => "/static/steam.png",
      "twitter" => "/static/twitter.png",
      _ => "",
    };

    let class = if let Some(b) = self.props.disable_invert {
      if !b {
        classes!("filter invert m-2".to_string())
      } else {
        classes!("m-2")
      }
    } else {
      classes!("m-2")
    };

    html! {
      <img class=class src={ icon } alt={ self.props.icon.clone() } width="24px" height="24px" />
    }
  }
}