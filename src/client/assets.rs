use crate::{protocol::{AssetFormat, DisplayFormat}, Request};

pub struct Assets<'a> {
    pub banner_asset: Vec<&'a DisplayFormat>,
    pub title_asset: Vec<&'a AssetFormat>,
    pub img_asset: Vec<&'a AssetFormat>,
    pub icon_asset: Vec<&'a AssetFormat>,
    pub thumb_asset: Vec<&'a AssetFormat>,
    pub video_asset: Vec<&'a AssetFormat>,
    pub data_asset: Vec<&'a AssetFormat>,
    pub html_asset: Vec<&'a AssetFormat>,
    pub video_cover_asset: Vec<&'a AssetFormat>,
    pub video_icon_asset: Vec<&'a AssetFormat>,
    pub video_end_img_asset: Vec<&'a AssetFormat>,
    pub video_end_title_asset: Vec<&'a AssetFormat>,
    pub video_end_button_img_asset: Vec<&'a AssetFormat>,
    pub video_end_button_text_asset: Vec<&'a AssetFormat>,
    pub video_end_html_asset: Vec<&'a AssetFormat>,
    pub asset_size: usize,
}

impl<'a> Assets<'a> {

    pub fn new(request: &'a Request) -> Self {
        let mut banner_asset: Vec<&'a DisplayFormat> = vec![];
        let mut title_asset: Vec<&'a AssetFormat> = vec![];
        let mut img_asset: Vec<&'a AssetFormat> = vec![];
        let mut icon_asset: Vec<&'a AssetFormat> = vec![];
        let mut thumb_asset: Vec<&'a AssetFormat> = vec![];
        let mut video_asset: Vec<&'a AssetFormat> = vec![];
        let mut data_asset: Vec<&'a AssetFormat> = vec![];
        let mut html_asset: Vec<&'a AssetFormat> = vec![];
        let mut video_cover_asset: Vec<&'a AssetFormat> = vec![];
        let mut video_icon_asset: Vec<&'a AssetFormat> = vec![];
        let mut video_end_img_asset: Vec<&'a AssetFormat> = vec![];
        let mut video_end_title_asset: Vec<&'a AssetFormat> = vec![];
        let mut video_end_button_img_asset: Vec<&'a AssetFormat> = vec![];
        let mut video_end_button_text_asset: Vec<&'a AssetFormat> = vec![];
        let mut video_end_html_asset: Vec<&'a AssetFormat> = vec![];
        let mut asset_size: usize = 0;

        match &request.item[0].spec.display.displayfmt {
            Some(displayfmt) => {
                banner_asset.push(displayfmt);
            },
            None => (),
        }
        match &request.item[0].spec.display.nativefmt {
            Some(nativefmt) => {
                for asset in &nativefmt.asset {
                    asset_size += 1;

                    match &asset.title {
                        Some(_) => {
                            title_asset.push(asset);
                        },
                        None => (),
                    }
                    match &asset.img {
                        Some(img) => {
                            match img.imagetype {
                                Some(1) => icon_asset.push(asset),
                                Some(3) => img_asset.push(asset),
                                Some(501) => thumb_asset.push(asset),
                                _ => (),
                            }
                            img_asset.push(asset);
                        },
                        None => (),
                    }
                    match &asset.video {
                        Some(video) => {
                            video_asset.push(asset);

                            match &video.comp {
                                Some(comp) => {
                                    for comp1 in comp {
                                        match comp1.vcm {
                                            Some(0) => {
                                                match &comp1.display.nativefmt {
                                                    Some(nativefmt) => {
                                                        for asset1 in &nativefmt.asset {
                                                            match &asset1.img {
                                                                Some(img) => {
                                                                    match img.imagetype {
                                                                        Some(1) => video_icon_asset.push(asset1),
                                                                        Some(3) => video_cover_asset.push(asset1),
                                                                        _ => (),
                                                                    }
                                                                }
                                                                None => (),
                                                            }
                                                        }
                                                    }
                                                    None => (),
                                                }
                                            },
                                            Some(1) => {
                                                match &comp1.display.nativefmt {
                                                    Some(nativefmt) => {
                                                        for asset1 in &nativefmt.asset {
                                                            match &asset1.img {
                                                                Some(img) => {
                                                                    match img.imagetype {
                                                                        Some(1) => video_end_button_img_asset.push(asset1),
                                                                        Some(3) => video_end_img_asset.push(asset1),
                                                                        _ => (),
                                                                    }
                                                                }
                                                                None => (),
                                                            }
                                                            match &asset1.title {
                                                                Some(_) => video_end_title_asset.push(asset1),
                                                                None => (),
                                                            }
                                                            match &asset1.data {
                                                                Some(data) => {
                                                                    if data.datatype == 12 {
                                                                        video_end_button_text_asset.push(asset1);
                                                                    }
                                                                }
                                                                None => (),
                                                            }
                                                            match &asset1.html {
                                                                Some(_) => video_end_html_asset.push(asset1),
                                                                None => (),
                                                            }
                                                        }
                                                    }
                                                    None => (),
                                                }
                                            },
                                            _ => (),
                                        }
                                    }
                                },
                                None => (),
                            }
                        },
                        None => (),
                    }
                    match &asset.data {
                        Some(_) => {
                            data_asset.push(asset);
                        },
                        None => (),
                    }
                    match &asset.html {
                        Some(_) => {
                            html_asset.push(asset);
                        },
                        None => (),
                    }
                }
            },
            None => (),
        }

        Self {
            banner_asset,
            title_asset,
            img_asset,
            icon_asset,
            thumb_asset,
            video_asset,
            data_asset,
            html_asset,
            video_cover_asset,
            video_icon_asset,
            video_end_img_asset,
            video_end_title_asset,
            video_end_button_img_asset,
            video_end_button_text_asset,
            video_end_html_asset,
            asset_size,
        }
    }

}
