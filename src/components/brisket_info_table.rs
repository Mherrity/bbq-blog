use chrono::{DateTime, Local, NaiveDateTime, Utc};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BrisketSupplier {
    Costco,
    WildFork,
    SnakeRiver,
}
impl BrisketSupplier {
    fn name(&self) -> AttrValue {
        match self {
            BrisketSupplier::Costco => AttrValue::from("Costco"),
            BrisketSupplier::WildFork => AttrValue::from("Wild Fork"),
            BrisketSupplier::SnakeRiver => AttrValue::from("Snake River Farms"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum BrisketGrade {
    Prime,
    Choice,
    Select,
}
impl BrisketGrade {
    fn name(&self) -> AttrValue {
        match self {
            BrisketGrade::Prime => AttrValue::from("Prime"),
            BrisketGrade::Choice => AttrValue::from("Choice"),
            BrisketGrade::Select => AttrValue::from("Select"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum BrisketRub {
    LoneStarRub,
}
impl BrisketRub {
    fn name(&self) -> AttrValue {
        match self {
            BrisketRub::LoneStarRub => AttrValue::from("Lone Star Rub"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BrisketInfo {
    pub name: AttrValue,
    pub supplier: BrisketSupplier,
    pub price: f64,
    pub weight: f64,
    pub grade: BrisketGrade,
    pub rub: BrisketRub,
    pub image: AttrValue,
    pub start_time: AttrValue,
}
impl BrisketInfo {
    pub fn formatted_price(&self) -> AttrValue {
        AttrValue::from(format!("${:.2}", self.price))
    }
    pub fn formatted_weight(&self) -> AttrValue {
        AttrValue::from(format!("{:.2} lbs", self.weight))
    }
}

#[function_component]
pub fn BrisketInfoTable(brisket: &BrisketInfo) -> Html {
    html! {
        <div>
            <div class="max-w-2xl mx-auto mt-8 p-4">
                <h2 class="text-3xl font-bold mb-4">{ brisket.name.clone() }</h2>

                <img src={brisket.image.clone()} alt="Brisket" class="w-full h-64 object-cover rounded-lg mb-4"/>

                <table class="w-full border-collapse border border-gray-300">
                    <tbody>
                        <tr class="bg-gray-100">
                            <td class="border border-gray-300 p-2 font-semibold">{ "Supplier" }</td>
                            <td class="border border-gray-300 p-2">{ brisket.supplier.name().clone() }</td>
                        </tr>
                        <tr>
                            <td class="border border-gray-300 p-2 font-semibold">{ "Price" }</td>
                            <td class="border border-gray-300 p-2">{ brisket.formatted_price().clone() }</td>
                        </tr>
                        <tr class="bg-gray-100">
                            <td class="border border-gray-300 p-2 font-semibold">{ "Weight" }</td>
                            <td class="border border-gray-300 p-2">{ brisket.formatted_weight().clone() }</td>
                        </tr>
                        <tr>
                            <td class="border border-gray-300 p-2 font-semibold">{ "Grade" }</td>
                            <td class="border border-gray-300 p-2">{ brisket.grade.name().clone() }</td>
                        </tr>
                         <tr>
                            <td class="border border-gray-300 p-2 font-semibold">{ "Rub" }</td>
                            <td class="border border-gray-300 p-2">{ brisket.rub.name().clone() }</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
