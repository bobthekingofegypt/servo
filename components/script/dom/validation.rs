/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use dom::bindings::str::DOMString;
use dom::element::Element;
use dom::bindings::codegen::Bindings::AttrBinding::AttrMethods;
use dom::bindings::codegen::Bindings::ElementBinding::ElementMethods;

pub trait Validatable {
    fn is_element_required(&self, element: &Element) -> bool {
        let attribute_name = DOMString::from("required");
        element.get_attribute_by_name(attribute_name).map(|s| s.Value()).is_some()
    }

    fn value_missing(&self) -> bool {
        return false;
    }

    fn value_too_short(&self) -> bool {
        return false;
    }

    fn value_too_long(&self) -> bool {
        return false;
    }
    
    fn value_pattern_mismatch(&self) -> bool {
        return false;
    }

    fn value_type_mismatch(&self) -> bool {
        return false;
    }

    fn valid(&self) -> bool {
        return !self.value_missing() && 
            !self.value_pattern_mismatch() &&
            !self.value_type_mismatch() &&
            !self.value_too_short() && 
            !self.value_too_long();
    }
}

pub fn minlength_value(element: &Element) -> Option<u32> {
    let attribute_name = atom!("minlength");
    if !element.has_attribute(&attribute_name) {
        return None;
    }
    
    return Some(element.get_uint_attribute(&attribute_name, 0));
}

pub fn maxlength_value(element: &Element) -> Option<i32> {
    let attribute_name = atom!("maxlength");
    if !element.has_attribute(&attribute_name) {
        return None;
    }
    
    println!("element value - {:?}", element.GetAttribute(DOMString::from("maxlength")));
    return Some(element.get_int_attribute(&attribute_name, 0));
}

pub fn pattern_value(element: &Element) -> Option<DOMString> {
    let attribute_name = atom!("pattern");
    if !element.has_attribute(&attribute_name) {
        return None;
    }
    
    return Some(element.get_string_attribute(&attribute_name));
}
