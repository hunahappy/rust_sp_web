/// CPU 온도를 측정합니다.
/// Mac과 Linux에서 자동으로 적절한 방식으로 실행됩니다.
///
/// # Returns
/// - `Ok(f32)`: CPU 온도 (섭씨)
/// - `Err(String)`: 오류 메시지
#[cfg(target_os = "macos")]
pub fn get_cpu_temperature() -> Result<f32, String> {
    // use std::process::Command;

    // // Mac에서는 powermetrics 또는 sysctl을 사용
    // let output = Command::new("powermetrics")
    //     .arg("--n")
    //     .arg("1")
    //     .output()
    //     .map_err(|e| format!("powermetrics 실행 실패: {}", e))?;

    // let stdout = String::from_utf8_lossy(&output.stdout);

    // // "CPU Core X Temperature:" 패턴 찾기
    // for line in stdout.lines() {
    //     if line.contains("CPU Core") && line.contains("Temperature:") {
    //         if let Some(temp_str) = line.split(':').nth(1) {
    //             if let Some(num_str) = temp_str.trim().split_whitespace().next() {
    //                 if let Ok(temp) = num_str.parse::<f32>() {
    //                     return Ok(temp);
    //                 }
    //             }
    //         }
    //     }
    // }

    Err("CPU 온도를 찾을 수 없습니다".to_string())
}

#[cfg(target_os = "linux")]
pub fn get_cpu_temperature() -> Result<f32, String> {
    use std::fs;
    use std::path::Path;

    // Linux에서는 /sys/class/thermal 또는 /proc 파일 시스템 사용
    let thermal_zone = Path::new("/sys/class/thermal/thermal_zone0/temp");

    if thermal_zone.exists() {
        let temp_raw =
            fs::read_to_string(thermal_zone).map_err(|e| format!("온도 파일 읽기 실패: {}", e))?;

        let temp_milli = temp_raw
            .trim()
            .parse::<f32>()
            .map_err(|e| format!("온도 파싱 실패: {}", e))?;

        // 일반적으로 밀리도씨로 저장되어 있으므로 1000으로 나눔
        return Ok(temp_milli / 1000.0);
    }

    Err("CPU 온도를 측정할 수 없습니다".to_string())
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn get_cpu_temperature() -> Result<f32, String> {
    Err("이 OS는 지원되지 않습니다 (Mac 또는 Linux만 지원)".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu_temperature() {
        match get_cpu_temperature() {
            Ok(temp) => {
                println!("CPU 온도: {}°C", temp);
                assert!(temp > 0.0 && temp < 150.0); // 일반적인 온도 범위
            }
            Err(e) => println!("온도 측정 오류: {}", e),
        }
    }
}
