use ext_php_rs::{php_function, php_class, php_impl};
use image::{imageops};
use ext_php_rs::prelude::PhpResult;



#[php_class]
pub struct ImageResize {
    source: String,
    width: u32,
    height: u32,

}


#[php_impl]
impl ImageResize {
  
    // No `#[constructor]` attribute required here - the name is `__construct`.
    pub fn __construct(source: String) -> PhpResult<Self> {
        
        let img = image::open(source.to_string()).map_err(|_| format!("Could not open file: {} !",source))?;

        Ok(Self { source, width: img.width(), height: img.height() })
    }

   

    #[getter]
    pub fn x(&self) -> u32 {
        self.width
    }

    #[getter]
    pub fn y(&self) -> u32 {
        self.height
    }



    // pub fn introduce(&self) {
    //     println!("My name is {} and I am {} years old. I live at {}.", self.name, self.age, self.address);
    // }

    // pub fn get_raw_obj(#[this] this: &mut ZendClassObject<Human>) {
    //     dbg!(this);   
    // }

    // pub fn get_max_age() -> i32 {
    //     Self::MAX_AGE
    // }


    pub fn crop(&mut self, width: u32, height: u32)  {
        self.width = width;
        self.height = height;
    }


    pub fn save(&self, target: String) -> PhpResult<bool> {
 

        let mut img = image::open(&self.source).map_err(|_| format!("Could not open file: {} !", &self.source))?;
    
        
        let subimg = imageops::resize(&mut img, self.x() ,self.y() , imageops::FilterType::Nearest);
    
    
        subimg.save(target).map_err(|_| format!("Could not open file: {} !", &self.source))?;
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
