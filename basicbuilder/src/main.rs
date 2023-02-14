

pub trait Builder {
    type OutputType;
    fn set_first_feature(& mut self, feature: i8);
    fn set_second_feature(& mut self, feature: i8);
    fn build(self) -> Self::OutputType;
}

#[derive(Debug)]
pub struct FirstObject {
    first_feature: i8,
    second_feature: i8,
    third_feature: i8
}

#[derive(Debug)]
pub struct SecondObject {
    first_feature: i8,
    second_feature: i8
}

#[derive(Default)]
pub struct FirstObjectBuilder{
    first_feature: Option<i8>,
    second_feature: Option<i8>
}

#[derive(Default)]
pub struct SecondObjectBuilder {
    first_feature: Option<i8>,
    second_feature: Option<i8>
}

impl Builder for FirstObjectBuilder {
    type OutputType = FirstObject;

    fn set_first_feature(& mut self, feature: i8) {
        self.first_feature = Some(feature)
    }

    fn set_second_feature(& mut self, feature: i8) {
        self.second_feature = Some(feature)
    }   

    fn build(self) -> FirstObject {
        FirstObject { first_feature: self.first_feature.expect("create first feature"),
            second_feature: self.second_feature.expect("create second feature"),
            third_feature: 5}
    }
}

impl Builder for SecondObjectBuilder {
    type OutputType = SecondObject;

    fn set_first_feature(& mut self, feature: i8) {
        self.first_feature = Some(feature)
    }

    fn set_second_feature(& mut self, feature: i8) {
        self.second_feature = Some(feature)
    }

    fn build(self) -> SecondObject {
        SecondObject { first_feature: self.first_feature.expect("create first feature"),
         second_feature: self.second_feature.expect("create second feature") }
    }
}

pub struct Caller;

impl Caller{
    pub fn build_first_object(builder: &mut impl Builder) {
        builder.set_first_feature(3);
        builder.set_second_feature(2);
    }

    pub fn build_second_object(builder: &mut impl Builder) {
        builder.set_first_feature(6);
        builder.set_second_feature(4);
    }
}

fn main() {
    let mut builder = FirstObjectBuilder::default();

    // Director gets the concrete builder object from the client
    // (application code). That's because application knows better which
    // builder to use to get a specific product.
    Caller::build_first_object(&mut builder);

    // The final product is often retrieved from a builder object, since
    // Director is not aware and not dependent on concrete builders and
    // products.
    let object: FirstObject = builder.build();
    println!("First object built: {:?}\n", object);

    let mut secondbuilder = SecondObjectBuilder::default();

    // Director may know several building recipes.
    Caller::build_second_object(&mut secondbuilder);

    // The final car manual.
    let second_object: SecondObject = secondbuilder.build();
    println!("Car manual built:\n{:?}", second_object);
}