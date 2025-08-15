use std::ops::{Deref, DerefMut};

use crate::errors::ReportsError;

#[derive(Debug, Default)]
pub struct Reports(pub Vec<Report>);

#[derive(Debug, Default, Clone)]
pub struct Report(pub Vec<Level>);

type Level = u32;

impl Reports {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_report_safe(&self, report: &Report) -> bool {
        let are_all_increasing = report.0.windows(2).all(|w| w[0] < w[1]);
        let are_all_decreasing = report.0.windows(2).all(|w| w[0] > w[1]);
        let are_distances_valid = report.0.windows(2).all(|w| w[1].abs_diff(w[0]) <= 3);
        (are_all_decreasing || are_all_increasing) && are_distances_valid
    }

    pub fn how_many_reports_are_safe(&self) -> usize {
        let mut safe_count = 0;
        for report in &self.0 {
            let mut report_is_safe = false;
            for i in 0..report.0.len() {
                let mut sub_report = report.clone();
                sub_report.0.remove(i);
                report_is_safe |= self.is_report_safe(&sub_report);
                if report_is_safe {
                    break;
                }
            }
            report_is_safe |= self.is_report_safe(report);
            if report_is_safe {
                safe_count += 1;
            }
        }
        safe_count
    }
}

impl Deref for Reports {
    type Target = Vec<Report>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Reports {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<Vec<Vec<u32>>> for Reports {
    type Error = ReportsError;
    fn try_from(data: Vec<Vec<u32>>) -> Result<Self, Self::Error> {
        let mut reports = Reports::default();
        for levels in data {
            let report = Report(levels.clone());
            reports.push(report);
        }
        Ok(reports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reports_creation() {
        let data = vec![vec![1, 2, 3], vec![4, 3, 2]];
        let reports: Reports = data.try_into().expect("Failed to create reports");
        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0].0, vec![1, 2, 3]);
        assert_eq!(reports[1].0, vec![4, 3, 2]);
    }
    #[test]
    fn test_how_many_reports_are_safe() {
        let data = vec![vec![1, 2, 3], vec![4, 3, 3], vec![1, 3, 5]];
        let reports: Reports = data.try_into().expect("Failed to create reports");
        assert_eq!(reports.how_many_reports_are_safe(), 3);
        let data = vec![vec![1, 2, 4], vec![5, 4, 0], vec![1, 3, 5]];
        let reports: Reports = data.try_into().expect("Failed to create reports");
        assert_eq!(reports.how_many_reports_are_safe(), 3);
        let data = vec![vec![1, 2, 3], vec![4, 4, 4], vec![7, 8, 9]];
        let reports: Reports = data.try_into().expect("Failed to create reports");
        assert_eq!(reports.how_many_reports_are_safe(), 2);
        let data = vec![vec![1, 2, 3], vec![4, 4, 1], vec![1, 7, 5]];
        let reports: Reports = data.try_into().expect("Failed to create reports");
        assert_eq!(reports.how_many_reports_are_safe(), 3);
    }
    #[test]
    fn test_reports_deref() {
        let data = vec![vec![1, 2, 3], vec![4, 3, 2]];
        let mut reports: Reports = data.try_into().expect("Failed to create reports");
        let report: &Report = &reports[0];
        assert_eq!(report.0, vec![1, 2, 3]);
        let report_mut: &mut Report = &mut reports[1];
        report_mut.0.push(5);
        assert_eq!(report_mut.0, vec![4, 3, 2, 5]);
    }
    #[test]
    fn test_reports_deref_mut() {
        let data = vec![vec![1, 2, 3], vec![4, 3, 2]];
        let mut reports: Reports = data.try_into().expect("Failed to create reports");
        reports.push(Report(vec![10, 20, 30]));
        assert_eq!(reports.len(), 3);
        assert_eq!(reports[2].0, vec![10, 20, 30]);
        reports[0].0.push(4);
        assert_eq!(reports[0].0, vec![1, 2, 3, 4]);
    }
    #[test]
    fn test_reports_try_from() {
        let data = vec![vec![1, 2, 3], vec![4, 3, 2]];
        let reports: Reports = data.try_into().expect("Failed to create reports");
        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0].0, vec![1, 2, 3]);
        assert_eq!(reports[1].0, vec![4, 3, 2]);
    }
    #[test]
    fn test_reports_try_from_empty() {
        let data: Vec<Vec<u32>> = vec![];
        let reports: Reports = data.try_into().expect("Failed to create reports");
        assert!(reports.is_empty());
    }
}
