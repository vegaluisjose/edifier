use crate::ast::*;

pub fn new_lut2() -> Cell {
    let diro = PortElements::Direction(PortDirection::Output);
    let porto = InterfacePort {
        name: "O".to_string(),
        element: diro,
    };

    let dirin = PortElements::Direction(PortDirection::Input);
    let porti0 = InterfacePort {
        name: "I0".to_string(),
        element: dirin,
    };

    let dirin = PortElements::Direction(PortDirection::Input);
    let porti1 = InterfacePort {
        name: "I1".to_string(),
        element: dirin,
    };

    let interface = CellInterface(vec![porto, porti0, porti1]);

    let cellview = CellView {
        name: "netlist".to_string(),
        interface,
        contents: CellContents(Vec::new()),
    };
    Cell {
        name: "LUT2".to_string(),
        views: CellViews(vec![cellview]),
    }
}

/*
(property INIT (string "4'h6"))
(property BOX_TYPE (string "PRIMITIVE"))
(property LOC (string "SLICE_X0Y0"))
(property BEL (string "A6LUT"))*/

pub fn lut2_prop_ini(val: String) -> Property{
    Property {
        name: "INIT".to_string(),
        property: PropertyValue::String(val),
    }
}

pub fn lut2_prop_box(val: String) -> Property{
    Property {
        name: "BOX_TYPE".to_string(),
        property: PropertyValue::String(val),
    }
}

pub fn lut2_prop_loc(val: String) -> Property{
    Property {
        name: "LOC".to_string(),
        property: PropertyValue::String(val),
    }
}

pub fn lut2_prop_bel(val: String) -> Property{
    Property {
        name: "BEL".to_string(),
        property: PropertyValue::String(val),
    }
}