use std::{
    fs,
    io::{ prelude::*, BufReader, BufWriter },
    path::Path,
    error::Error,
};

use chrono::prelude::*;

// TODO insert leading 0's in front of hex numbers that aren't 4 characters long
// ONLY DO THIS AFTER COSTUME RE-SAVING IS WORKING TO ENSURE IT DOESN'T BREAK FUNCTIONALITY
const VALIDITY_ID_CIPHER:  [u16; 256] = [
    0xBCD1, 0xBB65, 0x42C2, 0xDFFE, 0x9666, 0x431B, 0x8504, 0xEB46,
    0x6379, 0xD460, 0xCF14, 0x53CF, 0xDB51, 0xDB08, 0x12C8, 0xF602,
    0xE766, 0x2394, 0x250D, 0xDCBB, 0xA678, 0x2AF, 0xA5C6, 0x7EA6,
    0xB645, 0xCB4D, 0xC44B, 0xE5DC, 0x9FE6, 0x5B5C, 0x35F5, 0x701A,
    0x220F, 0x6C38, 0x1A56, 0x4CA3, 0xFFC6, 0xB152, 0x8D61, 0x7A58,
    0x9025, 0x8B3D, 0xBF0F, 0x95A3, 0xE5F4, 0xC127, 0x3BED, 0x320B,
    0xB7F3, 0x6054, 0x333C, 0xD383, 0x8154, 0x5242, 0x4E0D, 0xA94,
    0x7028, 0x8689, 0x3A22, 0x980, 0x1847, 0xB0F1, 0x9B5C, 0x4176,
    0xB858, 0xD542, 0x1F6C, 0x2497, 0x6A5A, 0x9FA9, 0x8C5A, 0x7743,
    0xA8A9, 0x9A02, 0x4918, 0x438C, 0xC388, 0x9E2B, 0x4CAD, 0x1B6,
    0xAB19, 0xF777, 0x365F, 0x1EB2, 0x91E, 0x7BF8, 0x7A8E, 0x5227,
    0xEAB1, 0x2074, 0x4523, 0xE781, 0x1A3, 0x163D, 0x3B2E, 0x287D,
    0x5E7F, 0xA063, 0xB134, 0x8FAE, 0x5E8E, 0xB7B7, 0x4548, 0x1F5A,
    0xFA56, 0x7A24, 0x900F, 0x42DC, 0xCC69, 0x2A0, 0xB22, 0xDB31,
    0x71FE, 0xC7D, 0x1732, 0x1159, 0xCB09, 0xE1D2, 0x1351, 0x52E9,
    0xF536, 0x5A4F, 0xC316, 0x6BF9, 0x8994, 0xB774, 0x5F3E, 0xF6D6,
    0x3A61, 0xF82C, 0xCC22, 0x9D06, 0x299C, 0x9E5, 0x1EEC, 0x514F,
    0x8D53, 0xA650, 0x5C6E, 0xC577, 0x7958, 0x71AC, 0x8916, 0x9B4F,
    0x2C09, 0x5211, 0xF6D8, 0xCAAA, 0xF7EF, 0x287F, 0x7A94, 0xAB49,
    0xFA2C, 0x7222, 0xE457, 0xD71A, 0xC3, 0x1A76, 0xE98C, 0xC037,
    0x8208, 0x5C2D, 0xDFDA, 0xE5F5, 0xB45, 0x15CE, 0x8A7E, 0xFCAD,
    0xAA2D, 0x4B5C, 0xD42E, 0xB251, 0x907E, 0x9A47, 0xC9A6, 0xD93F,
    0x85E, 0x35CE, 0xA153, 0x7E7B, 0x9F0B, 0x25AA, 0x5D9F, 0xC04D,
    0x8A0E, 0x2875, 0x4A1C, 0x295F, 0x1393, 0xF760, 0x9178, 0xF5B,
    0xFA7D, 0x83B4, 0x2082, 0x721D, 0x6462, 0x368, 0x67E2, 0x8624,
    0x194D, 0x22F6, 0x78FB, 0x6791, 0xB238, 0xB332, 0x7276, 0xF272,
    0x47EC, 0x4504, 0xA961, 0x9FC8, 0x3FDC, 0xB413, 0x7A, 0x806,
    0x7458, 0x95C6, 0xCCAA, 0x18D6, 0xE2AE, 0x1B06, 0xF3F6, 0x5050,
    0xC8E8, 0xF4AC, 0xC04C, 0xF41C, 0x992F, 0xAE44, 0x5F1B, 0x1113,
    0x1738, 0xD9A8, 0x19EA, 0x2D33, 0x9698, 0x2FE9, 0x323F, 0xCDE2,
    0x6D71, 0xE37D, 0xB697, 0x2C4F, 0x4373, 0x9102, 0x75D, 0x8E25,
    0x1672, 0xEC28, 0x6ACB, 0x86CC, 0x186E, 0x9414, 0xD674, 0xD1A5,
];

const COSTUME_SIGNATURE_DATA_SET_NUMBER: u8 = 0x78;
const COSTUME_SPECIFICATION_DATA_SET_NUMBER: u8 = 0xCA;
const APP_13_SEGMENT_MARKER: [u8; 2] = [0xFF, 0xED];
const APP_13_SEGMENT_ID: &[u8; 14] = b"Photoshop 3.0\0";
const APP_13_SEGMENT_SIGNATURE: &[u8; 4] = b"8BIM";
const APP_13_RESOURCE_ID: [u8; 2] = [0x04, 0x04];
const APP_13_RESOURCE_NAME: [u8; 2] = [0x00, 0x00];

// APP 13 Record DataSet
struct DataSet {
    // DataSet Tag
    tag_marker: u8,
    record_number: u8,
    data_set_number: u8,
    data_length: u16,

    // DataSet Data
    data: Vec<u8>,
}

impl DataSet {
    fn read(reader: &mut BufReader<fs::File>) -> std::io::Result<Self> {
        let mut tag_marker = [0u8; 1];
        let mut record_number = [0u8; 1];
        let mut data_set_number = [0u8; 1];
        let mut data_length = [0u8; 2];
        let mut data: Vec<u8>;

        reader.read_exact(&mut tag_marker)?;
        reader.read_exact(&mut record_number)?;
        reader.read_exact(&mut data_set_number)?;
        reader.read_exact(&mut data_length)?;

        let [ tag_marker ] = tag_marker;
        let [ record_number ] = record_number;
        let [ data_set_number ] = data_set_number;
        let data_length = unsafe { std::mem::transmute::<[u8; 2], u16>(data_length).to_be() };

        data = vec![0u8; data_length as usize];
        reader.read_exact(&mut data)?;

        Ok(Self {
            tag_marker,
            record_number,
            data_set_number,
            data_length,
            data,
        })
    }

    fn as_bytes(&self) -> Vec<u8> {
        [
            self.tag_marker.to_be_bytes().as_slice(),
            self.record_number.to_be_bytes().as_slice(),
            self.data_set_number.to_be_bytes().as_slice(),
            self.data_length.to_be_bytes().as_slice(),
            self.data.as_slice()
        ].concat()
    }
}

pub struct CostumeSave {
    first_segments: Vec<u8>,
    last_segments: Vec<u8>,
    datasets: Vec<DataSet>
}

impl CostumeSave {
    // TODO basic checks such as checking filetype for jpg, ensuring it's a valid costume from the
    // get-go, etc.
    // TODO refactor since there's probably a better way to do this
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = fs::File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut first_segments: Vec<u8> = vec![0u8; 2];
        let mut datasets: Vec<DataSet> = vec![];
        let mut last_segments: Vec<u8> = vec![];

        reader.read_exact(&mut first_segments)?; // read in SOI
        let mut app_13_segment_read = false;
        loop {
            let mut segment = [0u8; 2];
            let mut length_bytes = [0u8; 2];
            reader.read_exact(&mut segment)?;
            reader.read_exact(&mut length_bytes)?;
            // Subtract 2 because the length includes the 2 bytes used to indicate the length
            let length = unsafe { std::mem::transmute::<[u8; 2], u16>(length_bytes).to_be() } as usize - 2;
            // println!("Segment {:02X?} with length {}", segment, length);

            match segment {
                APP_13_SEGMENT_MARKER => {
                    let mut segment_id: Vec<u8> = vec![];
                    reader.read_until(0x00, &mut segment_id)?;
                    let segment_id = std::str::from_utf8(&segment_id).unwrap();
                    // println!("APP13 Segment ID: {}", segment_id);

                    let mut segment_signature = [0u8; 4];
                    reader.read_exact(&mut segment_signature)?;
                    let segment_signature = std::str::from_utf8(&segment_signature).unwrap();
                    // println!("APP13 Segment signature: {}", segment_signature);

                    // 0x0404 is IPTC-NAA record, contains "File Info..." information
                    let mut resource_id = [0u8; 2];
                    reader.read_exact(&mut resource_id)?;
                    // println!("Resource ID: {:04X?}", resource_id);

                    // Costume saves don't utilize the Resource Name field so we'll skip.
                    // 0x00 0x00 for null name.
                    reader.seek_relative(2)?;

                    let mut resource_length = [0u8; 4];
                    reader.read_exact(&mut resource_length)?;
                    let resource_length = unsafe { std::mem::transmute::<[u8; 4], u32>(resource_length).to_be() } as usize;
                    // println!("Resource length: {}", resource_length);

                    let current_position = reader.stream_position()?;
                    // Subtract 2 because the length includes the 2 bytes used to indicate the length
                    let end_of_segment = current_position + resource_length as u64 - 2;
                    while reader.stream_position()? < end_of_segment {
                        let dataset = DataSet::read(&mut reader)?;

                        // println!("\nDATASET");
                        // println!("Tag Marker: {:02X?}", dataset.tag_marker);
                        // println!("Record Number: {:02X?}", dataset.record_number);
                        // println!("Data Set Number: {:02X?}", dataset.data_set_number);
                        // println!("Data Length: {}", dataset.data_length);
                        // println!("Data: {}", unsafe { std::str::from_utf8_unchecked(&dataset.data) });

                        datasets.push(dataset);
                    }

                    app_13_segment_read = true;
                },
                _ => {
                    if !app_13_segment_read {
                        let mut data = vec![0u8; length];
                        reader.read_exact(&mut data)?;
                        first_segments.extend(segment.iter());
                        first_segments.extend(length_bytes.iter());
                        first_segments.extend(data.iter());
                    } else {
                        last_segments.extend(segment.iter());
                        last_segments.extend(length_bytes.iter());
                        reader.read_to_end(&mut last_segments)?;
                        break;
                    }
                }
            }
        }

        Ok(Self {
            first_segments,
            last_segments,
            datasets,
        })
    }

    pub fn write_to_file(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let mut writer = BufWriter::new(file);

        // Generate a new validity id and set our data
        let validity_id = self.generate_validity_id();
        let validity_id_dataset = &mut self.datasets.iter_mut()
            .filter(|DataSet { data_set_number, .. }| *data_set_number == COSTUME_SIGNATURE_DATA_SET_NUMBER)
            .nth(2)
            .unwrap();
        validity_id_dataset.data_length = validity_id.len() as u16;
        validity_id_dataset.data = validity_id.into();

        // Calculate APP13 length values
        let app_13_resource_data = self.datasets.iter().map(|ds| ds.as_bytes()).collect::<Vec<Vec<u8>>>().concat();
        let app_13_resource_length = app_13_resource_data.len() as u32;
        let app_13_segment_length: u16 = APP_13_SEGMENT_ID.len() as u16
            + APP_13_SEGMENT_SIGNATURE.len() as u16
            + APP_13_RESOURCE_ID.len() as u16
            + APP_13_RESOURCE_NAME.len() as u16
            + app_13_resource_length as u16;

        writer.write_all(&self.first_segments)?;

        // write APP13 segment
        writer.write_all(&APP_13_SEGMENT_MARKER)?;
        writer.write_all(&app_13_segment_length.to_be_bytes())?;
        writer.write_all(APP_13_SEGMENT_ID)?;
        writer.write_all(APP_13_SEGMENT_SIGNATURE)?;
        writer.write_all(&APP_13_RESOURCE_ID)?;
        writer.write_all(&APP_13_RESOURCE_NAME)?;
        writer.write_all(&app_13_resource_length.to_be_bytes())?;
        writer.write_all(&self.datasets.iter().map(|ds| ds.as_bytes()).collect::<Vec<Vec<u8>>>().concat())?;

        writer.write_all(&self.last_segments)?;

        Ok(())
    }

    pub fn set_character_name(&mut self, name: &str) {
        let character_name_dataset = &mut self.datasets.iter_mut()
            .filter(|DataSet { data_set_number, .. }| *data_set_number == COSTUME_SIGNATURE_DATA_SET_NUMBER)
            .nth(1)
            .unwrap();

        character_name_dataset.data_length = name.len() as u16;
        character_name_dataset.data = name.into();
    }

    pub fn get_character_name(&self) -> String {
        let character_name_dataset = &self.datasets.iter()
            .filter(|DataSet { data_set_number, .. }| *data_set_number == COSTUME_SIGNATURE_DATA_SET_NUMBER)
            .nth(1)
            .unwrap();

        std::str::from_utf8(&character_name_dataset.data).expect("Failed to parse utf8").to_string()
    }

    pub fn set_account_name(&mut self, name: &str) {
        let account_name_dataset = &mut self.datasets.iter_mut()
            .filter(|DataSet { data_set_number, .. }| *data_set_number == COSTUME_SIGNATURE_DATA_SET_NUMBER)
            .nth(0)
            .unwrap();

        account_name_dataset.data_length = name.len() as u16;
        account_name_dataset.data = name.into();
    }

    pub fn get_account_name(&mut self) -> String {
        let account_name_dataset = &mut self.datasets.iter_mut()
            .filter(|DataSet { data_set_number, .. }| *data_set_number == COSTUME_SIGNATURE_DATA_SET_NUMBER)
            .nth(0)
            .unwrap();

        std::str::from_utf8(&account_name_dataset.data).expect("Failed to parse utf8").to_string()
    }

    // TODO clean up this function
    // maybe have it return the i64 instead of a string
    fn generate_validity_id(&self) -> String {
        let costume_specification = &self.datasets.iter()
            .find(|DataSet { data_set_number, .. }| *data_set_number == COSTUME_SPECIFICATION_DATA_SET_NUMBER)
            .unwrap();

        let mut upper_bits = std::num::Wrapping(0u16);
        let mut lower_bits = std::num::Wrapping(0u16);

        for &byte in costume_specification.data.iter() {
            lower_bits += std::num::Wrapping(VALIDITY_ID_CIPHER[byte as usize]);
            upper_bits += lower_bits;
        }

        format!("7799{}\0", (upper_bits.0 as i32) << 16 | (lower_bits.0 as i32))
    }
}

const JAN_1_2000_UNIX_TIME: i64 = 946684800;
pub fn to_simulated_save_name(file: &Path, account_name: &str, character_name: &str) -> String {
    // If the last portion of a costume save's filename is a j2000 timestamp, separated from the rest
    // by an underscore (_), then the game will convert this to a datetime string to display alongside
    // the rest of the save name.
    let maybe_j2000_timestamp = file
        .file_stem().expect("Provided costume file path did not contain a file name")
        .to_str().expect("Failed to convert the file name from an OsString to &str")
        .split('_')
        .last().expect("No last item in split string???")
        .parse::<i64>();

    let datetime_string = match maybe_j2000_timestamp {
        Ok(j2000_timestamp) => {
            let unix_timestamp = j2000_timestamp + JAN_1_2000_UNIX_TIME;
            if let Some(utc_datetime) = NaiveDateTime::from_timestamp_opt(unix_timestamp, 0) {
                utc_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            } else {
                String::from("")
            }
        },
        Err(_) => String::from("")
    };

    format!("{}{} {}", account_name, character_name, datetime_string)
        .trim_end()
        .to_string()
}
