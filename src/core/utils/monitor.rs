// TODO: add ScreenOptions to the monitor to be able to change res/rate on the fly:
//      new options: Change current res, Change current rate
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Monitor {
    pub name: String,
    pub res: ScreenRes,
    pub rate: ScreenRate,
    pub is_primary: bool,
    pub is_auto: bool,
    pub pos: MonitorPosition,
    pub dupl: MonitorDuplicated,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MonitorPosition {
    pub is_related: bool,
    pub related_pos: String,
    pub related_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MonitorDuplicated {
    pub is_duplicated: bool,
    pub name: String,
}

impl ToString for Monitor {
    fn to_string(&self) -> String {
        let mut output = format!(
            "--output {} --mode {}x{} --rate {}",
            self.name, self.res.0, self.res.1, self.rate.0
        );
        if self.is_auto {
            output += " --auto";
        }
        if self.pos.is_related {
            write!(
                &mut output,
                " --{} {}",
                self.pos.related_pos, self.pos.related_name
            )
            .unwrap();
        }
        if self.dupl.is_duplicated {
            write!(&mut output, " --same-as {}", self.dupl.name).unwrap();
        }
        if self.is_primary {
            output += " --primary";
        }
        output
    }
}

impl MonitorPosition {
    pub fn new(related_pos: &str, related_name: &str) -> Self {
        Self {
            is_related: true,
            related_pos: related_pos.to_string(),
            related_name: related_name.to_string(),
        }
    }
}

impl MonitorDuplicated {
    pub fn new(name: &str) -> Self {
        Self {
            is_duplicated: true,
            name: name.to_string(),
        }
    }
}
