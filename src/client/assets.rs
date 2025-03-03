use std::collections::HashMap;

use crate::{protocol::{AssetFormat, DisplayFormat}, Request};

pub struct Assets<'a> {
    asset_size: usize,
    max_asset_id: i32,
    banner: Option<&'a DisplayFormat>,
    banner_index: usize,
    asset_map: HashMap<&'a str, Vec<&'a AssetFormat>>,
    asset_index: HashMap<&'a str, usize>,
}

impl<'a> Assets<'a> {

    pub fn new(request: &'a Request) -> Self {
        let mut assets = Self {
            asset_size: 0,
            max_asset_id: 0,
            banner: None,
            banner_index: 0,
            asset_map: HashMap::<&'a str, Vec<&'a AssetFormat>>::new(),
            asset_index: HashMap::<&'a str, usize>::new(),
        };

        match &request.item[0].spec.display.displayfmt {
            Some(displayfmt) => {
                assets.add_banner_format(displayfmt);
            },
            None => (),
        }
        match &request.item[0].spec.display.nativefmt {
            Some(nativefmt) => {
                for asset in &nativefmt.asset {
                    assets.asset_size += 1;
                    if asset.id > assets.max_asset_id {
                        assets.max_asset_id = asset.id;
                    }

                    match &asset.title {
                        Some(_) => {
                            assets.add_asset_format("title", asset);
                        },
                        None => (),
                    }
                    match &asset.img {
                        Some(img) => {
                            match img.imagetype {
                                Some(1) => {
                                    assets.add_asset_format("icon", asset);
                                },
                                Some(3) => {
                                    assets.add_asset_format("img", asset);
                                },
                                Some(501) => {
                                    assets.add_asset_format("thumb", asset);
                                },
                                _ => {
                                    assets.add_asset_format("img", asset);
                                },
                            }
                        },
                        None => (),
                    }
                    match &asset.video {
                        Some(video) => {
                            assets.add_asset_format("video", asset);

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
                                                                        Some(1) => {
                                                                            assets.add_asset_format("video#icon", asset);
                                                                        },
                                                                        Some(3) => {
                                                                            assets.add_asset_format("video#cover", asset);
                                                                        },
                                                                        _ => {
                                                                            assets.add_asset_format("video#cover", asset);
                                                                        },
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
                                                                        Some(1) => {
                                                                            assets.add_asset_format("video#end#button#img", asset);
                                                                        },
                                                                        Some(3) => {
                                                                            assets.add_asset_format("video#end#img", asset);
                                                                        },
                                                                        _ => {
                                                                            assets.add_asset_format("video#end#img", asset);
                                                                        },
                                                                    }
                                                                }
                                                                None => (),
                                                            }
                                                            match &asset1.title {
                                                                Some(_) => {
                                                                    assets.add_asset_format("video#end#title", asset);
                                                                },
                                                                None => (),
                                                            }
                                                            match &asset1.data {
                                                                Some(data) => {
                                                                    match data.datatype {
                                                                        12 => {
                                                                            assets.add_asset_format("video#end#button#text", asset);
                                                                        },
                                                                        _ => (),
                                                                    }
                                                                }
                                                                None => (),
                                                            }
                                                            match &asset1.html {
                                                                Some(_) => {
                                                                    assets.add_asset_format("video#end#html", asset);
                                                                },
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
                        Some(data) => {
                            assets.add_asset_format("data", asset);

                            match data.datatype {
                                1 => {
                                    assets.add_asset_format("data#sponsored", asset);
                                },
                                2 => {
                                    assets.add_asset_format("data#desc", asset);
                                },
                                3 => {
                                    assets.add_asset_format("data#rating", asset);
                                },
                                4 => {
                                    assets.add_asset_format("data#likes", asset);
                                },
                                5 => {
                                    assets.add_asset_format("data#downloads", asset);
                                },
                                6 => {
                                    assets.add_asset_format("data#price", asset);
                                },
                                7 => {
                                    assets.add_asset_format("data#saleprice", asset);
                                },
                                8 => {
                                    assets.add_asset_format("data#phone", asset);
                                },
                                9 => {
                                    assets.add_asset_format("data#address", asset);
                                },
                                10 => {
                                    assets.add_asset_format("data#desc2", asset);
                                },
                                11 => {
                                    assets.add_asset_format("data#displayurl", asset);
                                },
                                12 => {
                                    assets.add_asset_format("data#ctatext", asset);
                                },
                                501 => {
                                    assets.add_asset_format("data#comments", asset);
                                },
                                _ => (),
                            }
                        },
                        None => (),
                    }
                    match &asset.html {
                        Some(_) => {
                            assets.add_asset_format("html", asset);
                        },
                        None => (),
                    }
                }
            },
            None => (),
        }

        assets
    }

    pub fn add_banner_format(&mut self, banner: &'a DisplayFormat) {
        self.banner = Some(banner);
        self.banner_index = 0;
    }

    pub fn add_asset_format(&mut self, key: &'a str, asset: &'a AssetFormat) {
        if self.asset_map.contains_key(key) {
            self.asset_map.get_mut(key).unwrap().push(asset);
        } else {
            self.asset_map.insert(key, vec![asset]);
            self.asset_index.insert(key, 0);
        }
    }

    pub fn get_banner_size(&self) -> usize {
        if self.banner.is_some() {
            1
        } else {
            0
        }
    }

    pub fn get_asset_size(&self, key: &'a str) -> usize {
        if self.asset_map.contains_key(key) {
            let asset_vec = self.asset_map.get(key).unwrap();
            asset_vec.len()
        } else {
            0
        }
    }

    pub fn get_asset_total_size(&self) -> usize {
        self.asset_size
    }

    pub fn get_banner(&self) -> Option<&'a DisplayFormat> {
        self.banner
    }

    pub fn get_current_asset(&self, key: &'a str) -> Option<&'a AssetFormat> {
        if self.asset_map.contains_key(key) {
            let asset_vec = self.asset_map.get(key).unwrap();
            let index = *self.asset_index.get(key).unwrap();
            if asset_vec.len() > index {
                Some(asset_vec[index])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn consume_asset(&mut self, key: &'a str) -> i32 {
        let asset = self.get_current_asset(key);

        match asset {
            Some(asset) => {
                let index = *self.asset_index.get(key).unwrap();
                self.asset_index.insert(key, index + 1);
                asset.id
            },
            None => {
                self.max_asset_id += 1;
                self.max_asset_id
            }
        }
    }

}
