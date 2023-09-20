use ext_php_rs::{php_function, php_class, php_impl};
use image::{imageops, DynamicImage};
use ext_php_rs::prelude::PhpResult;

#[php_class]
pub struct ImageResize {
    source: DynamicImage,
    width: u32,
    height: u32,

}

#[php_impl]
impl ImageResize {
  
    // No `#[constructor]` attribute required here - the name is `__construct`.
    pub fn __construct(source: String) -> PhpResult<Self> {
        
        let img: image::DynamicImage = image::open(source.to_string()).map_err(|_| format!("Could not open file: {} !",source))?;

        let w = img.width();
        let h = img.height();
        Ok(Self { source: img, width: w, height: h })
    }

    #[getter]
    pub fn x(&self) -> u32 {
        self.width
    }

    #[getter]
    pub fn y(&self) -> u32 {
        self.height
    }

    pub fn crop(&mut self, width: u32, height: u32)  {
        self.width = width;
        self.height = height;
    }

    pub fn save(&self, target: String) -> PhpResult<bool> {
        let subimg = imageops::resize(&self.source, self.x() ,self.y() , imageops::FilterType::Nearest);
    
        subimg.save(&target).map_err(|_| format!("Could not open file: {} !", target))?;
        Ok(true)
    }
}


#[php_function]
fn resize(filename: String, target: String) -> String {
 

    let mut img = image::open(filename).unwrap();

    
    let subimg = imageops::resize(&mut img, 200,200, imageops::FilterType::Nearest);


    subimg.save(target).unwrap();
    "ok".to_string()
}
