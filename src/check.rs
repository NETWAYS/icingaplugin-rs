use std::fmt;

/// Represents a complete CheckResult from Icinga2's POV
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckResult {
    /// CheckResults know about their state (OK|Warning|Critical|Unknown)
    state: State,
    /// CheckResults know about their info string, to be displayed on stdout and Icingaweb2
    info: Option<String>,
    /// CheckResults know about their performance data, to be parsed by Icinga2 and displayed by
    /// Icingaweb2
    perf_data: Option<PerfData>,
}


impl fmt::Display for CheckResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(i) = &self.info {
            if let Some(pd) = &self.perf_data {
                write!(f, "{} - {} | {}", self.state, i, pd)
            } else {
                write!(f, "{} - {}", self.state, i)
            }
        } else if let Some(pd) = &self.perf_data {
            write!(f, "{} | {}", self.state, pd)
        } else {
            write!(f, "{}", self.state)
        }
    }
}


impl From<usize> for CheckResult {
    /// Allows building a `CheckResult` from its expected exit status
    ///
    /// # Arguments
    ///
    /// * `state` - the exit status this `CheckResult` will propagate
    ///
    /// # Examples
    ///
    /// ``` 
    /// use icingaplugin_rs::check::{CheckResult, State};
    /// assert_eq!(CheckResult::from(0).state(), State::OK);
    /// assert_eq!(CheckResult::from(115).state(), State::Unknown);
    fn from(state: usize) -> Self {
        match state {
            0 => CheckResult::new(State::OK),
            1 => CheckResult::new(State::Warning),
            2 => CheckResult::new(State::Critical),
            _ => CheckResult::new(State::Unknown),
        }
    }
}


impl CheckResult {
    /// Returns a new CheckResult with its `state` field initialized
    ///
    /// # Arguments
    ///
    /// * `state` - A State struct that represents the state of the check
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::{CheckResult, State};
    /// let check_result = CheckResult::new(State::OK);
    /// assert_eq!(check_result.to_string(), String::from("OK"));
    /// ```
    pub fn new(state: State) -> Self {
        Self {
            state: state,
            info: None,
            perf_data: None,
        }
    }

    /// Sets the info string for a CheckResult and returns the CheckResult
    ///
    /// # Arguments
    ///
    /// * `text` - An owned String, passed to the function
    ///
    /// # Examples
    ///
    /// ``` 
    /// use icingaplugin_rs::check::{CheckResult, State};
    /// let check_result = CheckResult::new(State::OK).set_info(String::from("Everything fine."));
    /// assert_eq!(check_result.to_string(), String::from("OK - Everything fine."));
    /// ```
    pub fn set_info(mut self, text: String) -> Self {
        self.info = Some(text);
        self
    }

    /// Sets the performance data for a CheckResult and returns the CheckResult
    /// 
    /// # Arguments
    ///
    /// * `pd` - a PerfData struct
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::{CheckResult, PerfData, Metric};
    /// let check_result = CheckResult::from(0).set_perf_data(PerfData::from_metric(
    /// Metric::new(String::from("label"), String::from("value"))));
    /// assert_eq!(check_result.to_string(), String::from("OK | 'label'=value;;;; "));
    /// ``` 
    pub fn set_perf_data(mut self, pd: PerfData) -> Self {
        self.perf_data = Some(pd);
        self
    }

    /// Returns the `state` field of a `CheckResult`
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::{CheckResult, State};
    /// assert_eq!(CheckResult::from(0).state(), State::OK);
    /// ```
    pub fn state(&self) -> State {
        self.state
    }

    /// Prints the formatted `CheckResult` and returns the corresponding exit code
    ///
    /// # Examples
    ///
    /// ``` 
    /// use icingaplugin_rs::check::CheckResult;
    /// assert_eq!(CheckResult::from(0).promote(), 0);
    pub fn promote(&self) -> i32 {
        println!("{}", self);

        self.state.into()
    }
}


/// A struct for collecting `metrics` to be embedded into a `CheckResult`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerfData {
    /// A list of `Metric` objects to propagate to Icinga2
    metrics: Vec<Metric>,
}


impl PerfData {
    /// Creates a new `PerfData` struct, wrapping a single `Metric`
    ///
    /// # Arguments
    ///
    /// * `single_metric` - a single Metric to be wrapped by the struct. It will be converted into
    /// a `Vec`
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::{PerfData, Metric};
    /// let metric = Metric::new(String::from("label"), String::from("value"));
    /// let pd = PerfData::from_metric(metric);
    /// assert_eq!(pd.to_string(), "'label'=value;;;; ");
    pub fn from_metric(single_metric: Metric) -> Self {
        Self {
            metrics: vec![single_metric],
        }
    }

    /// Creates a new `PerfData` struct, wrapping a collection of `metrics`
    ///
    /// # Arguments
    ///
    /// * `multiple_metrics` - a `Vec` containing a collection of `Metric` structurist
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::{PerfData, Metric};
    /// let metrics = vec![Metric::new(String::from("label1"), String::from("value1")),
    /// Metric::new(String::from("label2"), String::from("value2"))];
    /// assert_eq!(PerfData::from_metrics(metrics).to_string(), String::from("'label1'=value1;;;; 'label2'=value2;;;; "));
    pub fn from_metrics(multiple_metrics: Vec<Metric>) -> Self {
        Self {
            metrics: multiple_metrics,
        }
    }
}


impl fmt::Display for PerfData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for metric in &self.metrics {
            output += &(format!("{}", metric) + &String::from(" "));
        }
        write!(f, "{}", output)
    }
}


/// A struct representing performance metrics in a format parsable for Icinga2
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metric {
    /// the `Metric` name
    label: String,
    /// the `Metric` value
    value: String,
    /// the `warning` threshold, if specified
    warning: Option<String>,
    /// the `critical` threshold, if specified
    critical: Option<String>,
    /// the `min`imum `value`, obsolete if `value` is UOM='%'
    min: Option<String>,
    /// the `max`imum `value`, obsolete if `value` is UOM='%'
    max: Option<String>,
}


impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'={};{};{};{};{}", self.label, self.value,
               self.warning.as_ref().unwrap_or(&String::from("")),
               self.critical.as_ref().unwrap_or(&String::from("")),
               self.min.as_ref().unwrap_or(&String::from("")),
               self.max.as_ref().unwrap_or(&String::from(""))
               )
    }
}


impl Metric {
    /// Creates a new `Metric` struct with a `label` and `value`
    ///
    /// # Arguments
    ///
    /// * `label` - the name of this `Metric`
    /// * `value` - the value of this `Metric`
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::Metric;
    /// let metric = Metric::new(String::from("label"), String::from("value"));
    /// assert_eq!(metric.to_string(), String::from("'label'=value;;;;"));
    /// ```
    pub fn new(label: String, value: String) -> Self {
        Metric {
            label: label,
            value: value,
            warning: None,
            critical: None,
            min: None,
            max: None,
        }
    }

    /// Adds a `warning` threshold to the `Metric` struct
    ///
    /// # Arguments
    ///
    /// * `warning` - the threshold to be set 
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::Metric;
    /// let metric = Metric::new(String::from("label"),
    /// String::from("value")).warning(5.to_string());
    /// assert_eq!(metric.to_string(), String::from("'label'=value;5;;;"));
    /// ```
    pub fn warning(mut self, warning: String) -> Self {
        self.warning = Some(warning);
        self
    }

    /// Adds a `critical` threshold to the `Metric` struct
    ///
    /// # Arguments
    ///
    /// * `critical` - the threshold to be set 
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::Metric;
    /// let metric = Metric::new(String::from("label"),
    /// String::from("value")).critical(5.to_string());
    /// assert_eq!(metric.to_string(), String::from("'label'=value;;5;;"));
    /// ```
    pub fn critical(mut self, critical: String) -> Self {
        self.critical = Some(critical);
        self
    }

    /// Adds a `min` threshold to the `Metric` struct
    ///
    /// # Arguments
    ///
    /// * `min` - the threshold to be set 
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::Metric;
    /// let metric = Metric::new(String::from("label"),
    /// String::from("value")).min(5.to_string());
    /// assert_eq!(metric.to_string(), String::from("'label'=value;;;5;"));
    /// ```
    pub fn min(mut self, min: String) -> Self {
        self.min = Some(min);
        self
    }

    /// Adds a `max` threshold to the `Metric` struct
    ///
    /// # Arguments
    ///
    /// * `max` - the threshold to be set 
    ///
    /// # Examples
    ///
    /// ```
    /// use icingaplugin_rs::check::Metric;
    /// let metric = Metric::new(String::from("label"),
    /// String::from("value")).max(5.to_string());
    /// assert_eq!(metric.to_string(), String::from("'label'=value;;;;5"));
    /// ```
    pub fn max(mut self, max: String) -> Self {
        self.max = Some(max);
        self
    }
}


/// An enum representing check states known to Icinga2
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum State {
    OK,
    Warning,
    Critical,
    Unknown,
}


impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            State::OK => "OK",
            State::Warning => "WARNING",
            State::Critical => "CRITICAL",
            State::Unknown => "UNKNOWN",
        })
    }
}


impl From<State> for i32 {
    fn from(state: State) -> Self {
        match state {
            State::OK => 0,
            State::Warning => 1,
            State::Critical => 2,
            State::Unknown => 3,
        }
    }
}


impl From<State> for &str {
    fn from(state: State) -> Self {
        match state {
            State::OK => "OK",
            State::Warning => "WARNING",
            State::Critical => "CRITICAL",
            State::Unknown => "UNKNOWN",
        }
    }
}

impl From<State> for String {
    fn from(state: State) -> Self {
        match state {
            State::OK => String::from("OK"),
            State::Warning => String::from("WARNING"),
            State::Critical => String::from("CRITICAL"),
            State::Unknown => String::from("UNKNOWN"),
        }
    }
}
