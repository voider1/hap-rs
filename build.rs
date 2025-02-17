#[macro_use]
extern crate serde_json;

use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

use crypto::{digest::Digest, sha2::Sha256};
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError, Renderable};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    #[serde(rename = "Categories")]
    pub categories: Vec<Category>,
    #[serde(rename = "Characteristics")]
    pub characteristics: Vec<Characteristic>,
    #[serde(rename = "Services")]
    pub services: Vec<Service>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Category")]
    pub number: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Characteristic {
    #[serde(rename = "UUID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Constraints")]
    pub constraints: Option<Constraints>,
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "Properties")]
    pub properties: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Constraints {
    #[serde(rename = "ValidValues")]
    pub valid_values: Option<HashMap<String, String>>,
    #[serde(rename = "MaximumValue")]
    pub max_value: Option<serde_json::Value>,
    #[serde(rename = "MinimumValue")]
    pub min_value: Option<serde_json::Value>,
    #[serde(rename = "StepValue")]
    pub step_value: Option<serde_json::Value>,
    #[serde(rename = "MaximumLength")]
    pub max_len: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    #[serde(rename = "UUID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RequiredCharacteristics")]
    pub required_characteristics: Vec<String>,
    #[serde(rename = "OptionalCharacteristics")]
    pub optional_characteristics: Vec<String>,
}

fn if_eq_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars,
    c: &Context,
    rc: &mut RenderContext<'reg>,
    out: &mut Output,
) -> Result<(), RenderError> {
    let first = h.param(0).unwrap().value();
    let second = h.param(1).unwrap().value();
    let tmpl = if first == second { h.template() } else { h.inverse() };
    match tmpl {
        Some(ref t) => t.render(r, c, rc, out),
        None => Ok(()),
    }
}

fn trim_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        let trim = s.replace(" ", "").replace(".", "_");
        out.write(&trim)?;
    }
    Ok(())
}

fn file_name_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        let name = s.replace(" ", "_").replace(".", "_").to_lowercase();
        out.write(&name)?;
    }
    Ok(())
}

fn type_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "bool" => {
                out.write("bool")?;
            },
            "uint8" => {
                out.write("u8")?;
            },
            "uint16" => {
                out.write("u16")?;
            },
            "uint32" => {
                out.write("u32")?;
            },
            "uint64" => {
                out.write("u64")?;
            },
            "int" => {
                out.write("i32")?;
            },
            "int32" => {
                out.write("i32")?;
            },
            "float" => {
                out.write("f32")?;
            },
            "string" => {
                out.write("String")?;
            },
            "tlv8" => {
                out.write("Vec<u8>")?;
            },
            "data" => {
                out.write("Vec<u8>")?;
            },
            _ => {
                return Err(RenderError::new("Unknown Characteristic format"));
            },
        }
    }
    Ok(())
}

fn format_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "bool" => {
                out.write("Format::Bool")?;
            },
            "uint8" => {
                out.write("Format::UInt8")?;
            },
            "uint16" => {
                out.write("Format::UInt16")?;
            },
            "uint32" => {
                out.write("Format::UInt32")?;
            },
            "uint64" => {
                out.write("Format::UInt64")?;
            },
            "int" => {
                out.write("Format::Int32")?;
            },
            "int32" => {
                out.write("Format::Int32")?;
            },
            "float" => {
                out.write("Format::Float")?;
            },
            "string" => {
                out.write("Format::String")?;
            },
            "tlv8" => {
                out.write("Format::Tlv8")?;
            },
            "data" => {
                out.write("Format::Data")?;
            },
            _ => {
                return Err(RenderError::new("Unknown Characteristic format"));
            },
        }
    }
    Ok(())
}

fn unit_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "percentage" => {
                out.write("Unit::Percentage")?;
            },
            "arcdegrees" => {
                out.write("Unit::ArcDegrees")?;
            },
            "celsius" => {
                out.write("Unit::Celsius")?;
            },
            "lux" => {
                out.write("Unit::Lux")?;
            },
            "seconds" => {
                out.write("Unit::Seconds")?;
            },
            _ => {
                return Err(RenderError::new("Unknown Characteristic unit"));
            },
        }
    }
    Ok(())
}

fn uuid_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        out.write(&shorten_uuid(&s))?;
    }
    Ok(())
}

fn valid_values_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_object().unwrap();
    let mut output = String::from("vec![\n");
    for (key, val) in param {
        output.push_str(&format!("\t\t\t{}, // {}\n", key, val));
    }
    output.push_str("\t\t]");
    out.write(&output)?;
    Ok(())
}

fn perms_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let params = h.param(0).unwrap().value().as_array().unwrap();
    for param in params {
        match param.as_str() {
            Some("read") => {
                out.write("\n\t\t\tPerm::PairedRead,")?;
            },
            Some("write") => {
                out.write("\n\t\t\tPerm::PairedWrite,")?;
            },
            Some("cnotify") => {
                out.write("\n\t\t\tPerm::Events,")?;
            },
            _ => {},
        }
    }
    Ok(())
}

fn float_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let format = h.param(0).unwrap().value().as_str().unwrap();
    if format == "float" {
        out.write(" as f32")?;
    }
    Ok(())
}

fn shorten_uuid(id: &str) -> String {
    id.split("-").collect::<Vec<&str>>()[0]
        .trim_start_matches('0')
        .to_owned()
}

fn characteristic_name_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let id = h.param(0).unwrap().value().as_str().unwrap();
    let characteristics: Vec<Characteristic> = serde_json::from_value(h.param(1).unwrap().value().clone()).unwrap();
    for c in characteristics {
        if &c.id == id {
            let name = c.name.replace(" ", "").replace(".", "_");
            out.write(&name)?;
        }
    }
    Ok(())
}

fn characteristic_file_name_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let id = h.param(0).unwrap().value().as_str().unwrap();
    let characteristics: Vec<Characteristic> = serde_json::from_value(h.param(1).unwrap().value().clone()).unwrap();
    for c in characteristics {
        if &c.id == id {
            let name = c.name.replace(" ", "_").replace(".", "_").to_lowercase();
            out.write(&name)?;
        }
    }
    Ok(())
}

fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    let name = param.replace(" ", "_").replace(".", "_").to_lowercase();
    out.write(&name)?;
    Ok(())
}

static CATEGORIES: &'static str = "// THIS FILE IS AUTO-GENERATED\n
/// HAP Accessory category.
#[derive(Copy, Clone)]
pub enum Category {
{{#each Categories as |c|}}\
\t{{trim c.Name}} = {{c.Category}},
{{/each}}\
}
";

static HAP_TYPE: &'static str = "// THIS FILE IS AUTO-GENERATED\n
use serde::ser::{Serialize, Serializer};

/// HAP Service and Characteristic type.
#[derive(Copy, Clone, Debug)]
pub enum HapType {
    Unknown,
{{#each Characteristics as |c|}}\
\t{{trim c.Name}},
{{/each}}\
{{#each Services as |s|}}\
\t{{trim s.Name}},
{{/each}}\
}

impl HapType {
    /// Converts a `HapType` to its corresponding shortened UUID string.
    pub fn to_string(self) -> String {
        match self {
            HapType::Unknown => \"unknown\".into(),
{{#each Characteristics as |c|}}\
\t\t\tHapType::{{trim c.Name}} => \"{{uuid c.UUID}}\".into(),
{{/each}}\
{{#each Services as |s|}}\
\t\t\tHapType::{{trim s.Name}} => \"{{uuid s.UUID}}\".into(),
{{/each}}\
\t\t}
    }
}

impl Default for HapType {
    fn default() -> HapType { HapType::Unknown }
}

impl Serialize for HapType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
";

static CHARACTERISTIC: &'static str = "// THIS FILE IS AUTO-GENERATED\n
use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm{{#if characteristic.Unit}}, Unit{{/if}}};

/// {{characteristic.Name}} Characteristic.
pub type {{trim characteristic.Name}} = Characteristic<{{type characteristic.Format}}>;

/// Creates a new {{characteristic.Name}} Characteristic.
pub fn new() -> {{trim characteristic.Name}} {
    Characteristic::new(Inner::<{{type characteristic.Format}}> {
        hap_type: HapType::{{trim characteristic.Name}},
        format: {{format characteristic.Format}},
        perms: vec![{{perms characteristic.Properties}}
        ],\
        {{#if characteristic.Unit}}\n\t\tunit: Some({{unit characteristic.Unit}}),{{/if}}\
        {{#if characteristic.Constraints.MaximumValue includeZero=true}}\n\t\tmax_value: Some({{characteristic.Constraints.MaximumValue}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.MinimumValue includeZero=true}}\n\t\tmin_value: Some({{characteristic.Constraints.MinimumValue}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.StepValue includeZero=true}}\n\t\tstep_value: Some({{characteristic.Constraints.StepValue}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.MaximumLength includeZero=true}}\n\t\tmax_len: Some({{characteristic.Constraints.MaximumLength}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.MaximumDataLength includeZero=true}}\n\t\tmax_data_len: Some({{characteristic.Constraints.MaximumDataLength}}{{float characteristic.Format}}),{{/if}}\
        {{#if characteristic.Constraints.ValidValues includeZero=true}}\n\t\tvalid_values: Some({{valid_values characteristic.Constraints.ValidValues}}),{{/if}}
        ..Default::default()
    })
}
";

static CHARACTERISTIC_MOD: &'static str = "// THIS FILE IS AUTO-GENERATED
{{#each characteristics as |c|}}\npub mod {{c}};{{/each}}
";

static SERVICE: &'static str = "// THIS FILE IS AUTO-GENERATED\n
use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\t{{characteristic_file_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
{{#each service.OptionalCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\t{{characteristic_file_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
\t},
    HapType,
};

/// {{service.Name}} Service.
pub type {{trim service.Name}} = Service<{{trim service.Name}}Inner>;

impl Default for {{trim service.Name}} {
    fn default() -> {{trim service.Name}} { new() }
}

/// Inner type of the {{service.Name}} Service.
#[derive(Default)]
pub struct {{trim service.Name}}Inner {
    /// ID of the {{service.Name}} Service.
    id: u64,
    /// `HapType` of the {{service.Name}} Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t/// {{c.Name}} Characteristic.
\tpub {{characteristic_file_name r ../../this.characteristics}}: {{characteristic_file_name r ../../this.characteristics}}::{{characteristic_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
\n{{#each service.OptionalCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t/// {{c.Name}} Characteristic.
\tpub {{characteristic_file_name r ../../this.characteristics}}: Option<{{characteristic_file_name r ../../this.characteristics}}::{{characteristic_name r ../../this.characteristics}}>,
{{/if_eq}}\
{{/each}}\
{{/each}}\
}

impl HapService for {{trim service.Name}}Inner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\t\t&self.{{characteristic_file_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\t];
{{#each service.OptionalCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\tif let Some(c) = &self.{{characteristic_file_name r ../../this.characteristics}} {
\t\t    characteristics.push(c);
\t\t}
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\tcharacteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\t\t&mut self.{{characteristic_file_name r ../../this.characteristics}},
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\t];
{{#each service.OptionalCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\tif let Some(c) = &mut self.{{characteristic_file_name r ../../this.characteristics}} {
\t\t    characteristics.push(c);
\t\t}
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\tcharacteristics
    }
}

/// Creates a new {{service.Name}} Service.
pub fn new() -> {{trim service.Name}} {
    {{trim service.Name}}::new({{trim service.Name}}Inner {
        hap_type: HapType::{{trim service.Name}},
{{#each service.RequiredCharacteristics as |r|}}\
{{#each ../this.characteristics as |c|}}\
{{#if_eq r c.UUID}}\
\t\t{{characteristic_file_name r ../../this.characteristics}}: {{characteristic_file_name r ../../this.characteristics}}::new(),
{{/if_eq}}\
{{/each}}\
{{/each}}\
        \t\t..Default::default()
    })
}
";

static SERVICE_MOD: &'static str = "// THIS FILE IS AUTO-GENERATED
{{#each services as |s|}}\npub mod {{s}};{{/each}}
";

static ACCESSORY: &'static str = "// THIS FILE IS AUTO-GENERATED\n
use crate::{
\taccessory::{HapAccessory, HapAccessoryService, Accessory, Information},
\tservice::{HapService, accessory_information::AccessoryInformation, {{snake_case service.Name}}},
\tevent::EventEmitterPtr,
\tResult,
};

/// {{service.Name}} Accessory.
pub type {{trim service.Name}} = Accessory<{{trim service.Name}}Inner>;

/// Inner type of the {{service.Name}} Accessory.
#[derive(Default)]
pub struct {{trim service.Name}}Inner {
    /// ID of the {{service.Name}} Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// {{service.Name}} Service.
    pub {{snake_case service.Name}}: {{snake_case service.Name}}::{{trim service.Name}},
}

impl HapAccessory for {{trim service.Name}}Inner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.{{snake_case service.Name}},
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.{{snake_case service.Name}},
        ]
    }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation {
        &mut self.accessory_information
    }

    fn init_iids(&mut self, accessory_id: u64, event_emitter: EventEmitterPtr) -> Result<()> {
        let mut next_iid = 1;
        for service in self.get_mut_services() {
            service.set_id(next_iid);
            next_iid += 1;
            for characteristic in service.get_mut_characteristics() {
                characteristic.set_id(next_iid)?;
                characteristic.set_accessory_id(accessory_id)?;
                characteristic.set_event_emitter(Some(event_emitter.clone()))?;
                next_iid += 1;
            }
        }
        Ok(())
    }
}

/// Creates a new {{service.Name}} Accessory.
pub fn new(information: Information) -> Result<{{trim service.Name}}> {
    let mut {{snake_case service.Name}} = {{snake_case service.Name}}::new();
    {{snake_case service.Name}}.set_primary(true);
    Ok({{trim service.Name}}::new({{trim service.Name}}Inner {
        accessory_information: information.to_service()?,
        {{snake_case service.Name}},
        ..Default::default()
    }))
}
";

static ACCESSORY_MOD: &'static str = "// THIS FILE IS AUTO-GENERATED
{{#each accessories as |a|}}\npub mod {{a}};{{/each}}
";

fn main() {
    let mut metadata_file = File::open("default.metadata.json").unwrap();
    let mut metadata_hash_file = OpenOptions::new().read(true).open("metadata_hash").unwrap();

    let mut buf = Vec::new();
    let mut cached_hash = String::new();

    metadata_file.read_to_end(&mut buf).unwrap();
    metadata_hash_file.read_to_string(&mut cached_hash).unwrap();

    let mut hasher = Sha256::new();
    hasher.input(&buf);
    let current_hash = hasher.result_str();

    if cached_hash == current_hash {
        return;
    }

    let mut metadata_hash_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("metadata_hash")
        .unwrap();
    metadata_hash_file.write_all(&current_hash.as_bytes()).unwrap();

    let metadata: Metadata = serde_json::from_slice(&buf).unwrap();

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("if_eq", Box::new(if_eq_helper));
    handlebars.register_helper("trim", Box::new(trim_helper));
    handlebars.register_helper("file_name", Box::new(file_name_helper));
    handlebars.register_helper("format", Box::new(format_helper));
    handlebars.register_helper("type", Box::new(type_helper));
    handlebars.register_helper("unit", Box::new(unit_helper));
    handlebars.register_helper("uuid", Box::new(uuid_helper));
    handlebars.register_helper("valid_values", Box::new(valid_values_helper));
    handlebars.register_helper("perms", Box::new(perms_helper));
    handlebars.register_helper("float", Box::new(float_helper));
    handlebars.register_helper("characteristic_name", Box::new(characteristic_name_helper));
    handlebars.register_helper("characteristic_file_name", Box::new(characteristic_file_name_helper));
    handlebars.register_helper("snake_case", Box::new(snake_case_helper));
    handlebars.register_template_string("categories", CATEGORIES).unwrap();
    handlebars.register_template_string("hap_type", HAP_TYPE).unwrap();
    handlebars
        .register_template_string("characteristic", CHARACTERISTIC)
        .unwrap();
    handlebars
        .register_template_string("characteristic_mod", CHARACTERISTIC_MOD)
        .unwrap();
    handlebars.register_template_string("service", SERVICE).unwrap();
    handlebars.register_template_string("service_mod", SERVICE_MOD).unwrap();
    handlebars.register_template_string("accessory", ACCESSORY).unwrap();
    handlebars
        .register_template_string("accessory_mod", ACCESSORY_MOD)
        .unwrap();

    let categories = handlebars.render("categories", &metadata).unwrap();
    let categories_path = "src/accessory/category.rs".to_owned();
    let mut categories_file = File::create(&categories_path).unwrap();
    categories_file.write_all(categories.as_bytes()).unwrap();

    let hap_type = handlebars.render("hap_type", &metadata).unwrap();
    let hap_type_path = "src/hap_type.rs".to_owned();
    let mut hap_type_file = File::create(&hap_type_path).unwrap();
    hap_type_file.write_all(hap_type.as_bytes()).unwrap();

    let characteristics_base_path = "src/characteristic/generated/";
    fs::remove_dir_all(&characteristics_base_path).unwrap();
    fs::create_dir_all(&characteristics_base_path).unwrap();
    let mut characteristsic_names = vec![];
    for c in &metadata.characteristics {
        let characteristic = handlebars
            .render("characteristic", &json!({ "characteristic": c }))
            .unwrap();
        let characteristic_file_name = c.name.replace(" ", "_").replace(".", "_").to_lowercase();
        let mut characteristic_path = String::from(characteristics_base_path);
        characteristic_path.push_str(&characteristic_file_name);
        characteristic_path.push_str(".rs");
        let mut characteristic_file = File::create(&characteristic_path).unwrap();
        characteristic_file.write_all(characteristic.as_bytes()).unwrap();
        characteristsic_names.push(characteristic_file_name);
    }
    let characteristic_mod = handlebars
        .render(
            "characteristic_mod",
            &json!({ "characteristics": characteristsic_names }),
        )
        .unwrap();
    let mut characteristic_mod_file = File::create(&format!("{}mod.rs", characteristics_base_path)).unwrap();
    characteristic_mod_file
        .write_all(characteristic_mod.as_bytes())
        .unwrap();

    let services_base_path = "src/service/generated/";
    let accessory_base_path = "src/accessory/generated/";
    fs::remove_dir_all(&services_base_path).unwrap();
    fs::remove_dir_all(&accessory_base_path).unwrap();
    fs::create_dir_all(&services_base_path).unwrap();
    fs::create_dir_all(&accessory_base_path).unwrap();
    let mut service_names = vec![];
    let mut accessory_names = vec![];
    for s in &metadata.services {
        let service = handlebars
            .render(
                "service",
                &json!({"service": s, "characteristics": &metadata.characteristics}),
            )
            .unwrap();
        let service_file_name = s.name.replace(" ", "_").replace(".", "_").to_lowercase();
        let mut service_path = String::from(services_base_path);
        service_path.push_str(&service_file_name);
        service_path.push_str(".rs");
        let mut service_file = File::create(&service_path).unwrap();
        service_file.write_all(service.as_bytes()).unwrap();
        service_names.push(service_file_name.clone());

        if s.name != "Accessory Information"
            && s.name != "Battery Service"
            && s.name != "Camera RTP Stream Management"
            && s.name != "Doorbell"
            && s.name != "Faucet"
            && s.name != "Filter Maintenance"
            && s.name != "Irrigation System"
            && s.name != "Lock Management"
            && s.name != "Lock Mechanism"
            && s.name != "Microphone"
            && s.name != "Service Label"
            && s.name != "Slat"
            && s.name != "Speaker"
            && s.name != "Television"
        {
            let accessory = handlebars
                .render(
                    "accessory",
                    &json!({"service": s, "characteristics": &metadata.characteristics}),
                )
                .unwrap();
            let mut accessory_path = String::from(accessory_base_path);
            accessory_path.push_str(&service_file_name);
            accessory_path.push_str(".rs");
            let mut accessory_file = File::create(&accessory_path).unwrap();
            accessory_file.write_all(accessory.as_bytes()).unwrap();
            accessory_names.push(service_file_name);
        }
    }
    let service_mod = handlebars
        .render("service_mod", &json!({ "services": service_names }))
        .unwrap();
    let mut service_mod_file = File::create(&format!("{}mod.rs", services_base_path)).unwrap();
    service_mod_file.write_all(service_mod.as_bytes()).unwrap();
    let accessory_mod = handlebars
        .render("accessory_mod", &json!({ "accessories": accessory_names }))
        .unwrap();
    let mut accessory_mod_file = File::create(&format!("{}mod.rs", accessory_base_path)).unwrap();
    accessory_mod_file.write_all(accessory_mod.as_bytes()).unwrap();
}
