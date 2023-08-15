use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::error::Result;

const LABEL_RE: Lazy<std::result::Result<Regex, regex::Error>> =
    Lazy::new(|| Regex::new(r"#label \{([^}]+)\}"));
const METRIC_RE: Lazy<std::result::Result<Regex, regex::Error>> =
    Lazy::new(|| Regex::new(r"#metric (\w+)\s+(\w+) \{([^}]+)\}\s+(.+)"));

pub trait LabelExtractor {
    fn extract_label(&self) -> Result<Vec<(String, String)>>;
}

impl LabelExtractor for String {
    fn extract_label(&self) -> Result<Vec<(String, String)>> {
        let mut labels = vec![];
        let label_re = LABEL_RE;
        let re = label_re.as_ref().map_err(|e| e.to_owned())?;
        self.lines().for_each(|line| {
            if line.starts_with("#label") {
                let captures = re.captures(line);
                if let Some(captures) = captures {
                    let line_labels = captures[1]
                        .split(',')
                        .map(|item| {
                            let mut iter = item.split('=');
                            let key = iter.next().unwrap_or("").trim().to_string();
                            let value = iter.next().unwrap_or("").trim().to_string();
                            (key, value)
                        })
                        .collect::<Vec<(String, String)>>();
                    labels.extend(line_labels);
                }
            }
        });
        Ok(labels)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MetricValue {
    F64(f64),
    U64(u64),
}
/// #metric minio_bucket_usage_total_bytes gauge {bucket=x,server="node0.minio.com:9000"} 9.78413036153943e+14
/// #metric minio_request_times counter {bucket=x,server="node0.minio.com:9000",type="get"} 1
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MetricStruct {
    pub name: String,
    pub typ: String,
    pub labels: Vec<(String, String)>,
    pub value: MetricValue,
}

impl From<MetricValue> for f64 {
    fn from(value: MetricValue) -> Self {
        match value {
            MetricValue::F64(v) => v,
            MetricValue::U64(v) => v as f64,
        }
    }
}

impl From<MetricValue> for u64 {
    fn from(value: MetricValue) -> Self {
        match value {
            MetricValue::U64(v) => v,
            MetricValue::F64(v) => v as u64,
        }
    }
}
impl MetricStruct {
    pub fn send(&self) {
        match self.typ.as_str() {
            "gauge" => {
                let k = format!("{}{}", "sertus_", self.name);
                let v: f64 = self.value.clone().into();
                metrics::gauge!(k, v, &self.labels);
            }
            "counter" => {
                let v: u64 = self.value.clone().into();
                metrics::counter!(self.name.clone(), v, &self.labels);
            }
            _ => {
                warn!("unknown metric type: {}", self.typ);
            }
        }
    }
}

pub trait MetricExtractor {
    fn extract_metric(&self) -> Result<Vec<MetricStruct>>;
}

impl MetricExtractor for String {
    fn extract_metric(&self) -> Result<Vec<MetricStruct>> {
        let mut metrics = vec![];
        let metric_re = METRIC_RE;
        let re = metric_re.as_ref().map_err(|e| e.to_owned())?;
        self.lines().for_each(|line| {
            if line.starts_with("#metric") {
                let captures = re.captures(line);
                if let Some(captures) = captures {
                    let name = captures[1].to_string();
                    let typ = captures[2].to_string();
                    let labels = captures[3]
                        .split(',')
                        .map(|item| {
                            let mut iter = item.split('=');
                            let key = iter.next().unwrap_or("").trim().to_string();
                            let value = iter.next().unwrap_or("").trim().to_string();
                            (key, value)
                        })
                        .collect::<Vec<(String, String)>>();

                    let value = match typ.as_str() {
                        "gauge" => {
                            let value = captures[4].parse::<f64>().unwrap_or(0.0);
                            MetricValue::F64(value)
                        }
                        "counter" => {
                            let value = captures[4].parse::<u64>().unwrap_or(0);
                            MetricValue::U64(value)
                        }
                        _ => {
                            warn!("unknown metric type: {}", typ);
                            return;
                        }
                    };
                    let metric = MetricStruct {
                        name,
                        typ,
                        labels,
                        value,
                    };
                    metrics.push(metric);
                }
            }
        });
        Ok(metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_label() {
        let s = r#"#label {k=v, x=y}"#.to_string();
        let labels = s.extract_label().unwrap();
        assert_eq!(
            labels,
            vec![
                ("k".to_string(), "v".to_string()),
                ("x".to_string(), "y".to_string())
            ]
        );
    }
    #[test]
    fn test_extract_metric() {
        let s = r#"
#metric xxx gauge {k=v, x=y} 1.0
#metric xxx counter {k=v} 1
            "#
        .to_string();
        let metrics = s.extract_metric().unwrap();
        assert_eq!(
            metrics,
            vec![
                MetricStruct {
                    name: "xxx".to_string(),
                    typ: "gauge".to_string(),
                    labels: vec![
                        ("k".to_string(), "v".to_string()),
                        ("x".to_string(), "y".to_string())
                    ],
                    value: MetricValue::F64(1.0)
                },
                MetricStruct {
                    name: "xxx".to_string(),
                    typ: "counter".to_string(),
                    labels: vec![("k".to_string(), "v".to_string()),],
                    value: MetricValue::U64(1)
                }
            ]
        );
    }
}
