use alloc::collections::BTreeMap;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use ffxiv_ex::{Action, BNpcName, ClassJob, CraftAction, ENpcResident, Item, NamedExRow, PlaceName, Quest, WrappedEx};
use ffxiv_parser::Language;
use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

use crate::list::List;

pub struct App {
    link: ComponentLink<Self>,
    data: Option<BTreeMap<u32, Vec<String>>>,
}

pub enum Msg {
    OnDisplay(&'static str),
    OnDataReady(BTreeMap<u32, Vec<String>>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, data: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnDisplay(x) => {
                self.display_result(x);
                true
            }
            Msg::OnDataReady(x) => {
                self.data = Some(x);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let buttons = [
            "classjob",
            "item",
            "action",
            "craftaction",
            "bnpcname",
            "enpcresident",
            "quest",
            "placename",
        ]
        .iter()
        .map(|x| {
            html! {
                <button onclick=self.link.callback(move |_| Msg::OnDisplay(x))>{ x }</button>
            }
        })
        .collect::<Html>();

        html! {
            <div>
                <span>
                    { buttons }
                </span>
                <List data = &self.data>
                </List>
            </div>
        }
    }
}

impl App {
    fn display_result(&self, name: &'static str) {
        let callback = self.link.callback(Msg::OnDataReady);

        spawn_local(async move {
            let regions = vec![
                (
                    "global_525",
                    vec![Language::Japanese, Language::English, Language::Deutsch, Language::French],
                ),
                ("chn_511", vec![Language::ChineseSimplified]),
                ("kor_510", vec![Language::Korean]),
            ];

            let mut result = BTreeMap::new();

            for (uri, languages) in regions {
                let names = match name {
                    "classjob" => Self::read_names::<ClassJob>(uri, &languages).await,
                    "item" => Self::read_names::<Item>(uri, &languages).await,
                    "action" => Self::read_names::<Action>(uri, &languages).await,
                    "craftaction" => Self::read_names::<CraftAction>(uri, &languages).await,
                    "bnpcname" => Self::read_names::<BNpcName>(uri, &languages).await,
                    "enpcresident" => Self::read_names::<ENpcResident>(uri, &languages).await,
                    "quest" => Self::read_names::<Quest>(uri, &languages).await,
                    "placename" => Self::read_names::<PlaceName>(uri, &languages).await,
                    _ => panic!(),
                }
                .unwrap();

                for (k, mut v) in names {
                    result.entry(k).or_insert_with(Vec::new).append(&mut v);
                }
            }

            callback.emit(result);
        });
    }

    async fn read_names<'a, T: NamedExRow<'static> + 'static>(uri: &str, languages: &[Language]) -> Result<BTreeMap<u32, Vec<String>>> {
        let provider = ExtractedFileProviderWeb::new(&format!("https://ffxiv-data.dlunch.net/compressed/{}/", uri));
        let package = SqPackReaderExtractedFile::new(provider);

        let wrapped_ex = WrappedEx::<T>::new(&package).await?;
        // TODO do we really require unsafe here??
        let wrapped_ex_ref: &WrappedEx<T> = unsafe { core::mem::transmute(&wrapped_ex) };

        let mut result = BTreeMap::<u32, Vec<_>>::new();

        for language in languages {
            let all = wrapped_ex_ref.all(*language).unwrap();

            for (k, v) in all {
                result.entry(k).or_insert_with(Vec::new).push(v.name());
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_read_name() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let _ = App::read_names::<Item>(
            "global_525",
            &[Language::Japanese, Language::English, Language::Deutsch, Language::French],
        )
        .await?;

        Ok(())
    }
}
