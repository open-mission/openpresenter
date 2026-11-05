use serde::{Deserialize, Serialize};

/// Telemetria de hardware e recursos do sistema e da aplicação.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Memória física (RSS) utilizada pelo processo do OpenPresenter em bytes.
    pub process_memory_bytes: u64,
    /// Memória física utilizada pelo processo do OpenPresenter em Megabytes.
    pub process_memory_mb: f64,
    /// Carga percentual de CPU utilizada pelo processo do OpenPresenter (0.0 a 100.0%).
    pub process_cpu_percent: f32,

    /// Memória RAM total disponível no computador em bytes.
    pub system_memory_total_bytes: u64,
    /// Memória RAM atualmente utilizada por todos os processos da máquina em bytes.
    pub system_memory_used_bytes: u64,
    /// Memória RAM total da máquina em Megabytes.
    pub system_memory_total_mb: f64,
    /// Memória RAM utilizada da máquina em Megabytes.
    pub system_memory_used_mb: f64,
    /// Percentual de uso da memória RAM da máquina (0.0 a 100.0%).
    pub system_memory_percent: f32,

    /// Carga global de processamento de todos os núcleos de CPU da máquina (0.0 a 100.0%).
    pub system_cpu_percent: f32,

    /// Modelo / Fabricante da GPU detectada (ex.: "Apple M1", "NVIDIA GeForce RTX 3060").
    pub gpu_model: Option<String>,
    /// Percentual de utilização da GPU (quando suportado pelo driver/sistema operacional).
    pub gpu_usage_percent: Option<f32>,
    /// Quantidade de memória de vídeo dedicada (VRAM) utilizada em Megabytes (quando aplicável).
    pub gpu_memory_used_mb: Option<f64>,
    /// Quantidade de memória de vídeo dedicada (VRAM) total em Megabytes (quando aplicável).
    pub gpu_memory_total_mb: Option<f64>,

    /// Tempo de atividade ininterrupta do servidor OpenPresenter em segundos.
    pub uptime_seconds: u64,
    /// Timestamp Unix (em milissegundos) da medição.
    pub timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_metrics_serialization() {
        let metrics = SystemMetrics {
            process_memory_bytes: 47_185_920,
            process_memory_mb: 45.0,
            process_cpu_percent: 1.5,
            system_memory_total_bytes: 17_179_869_184,
            system_memory_used_bytes: 8_589_934_592,
            system_memory_total_mb: 16384.0,
            system_memory_used_mb: 8192.0,
            system_memory_percent: 50.0,
            system_cpu_percent: 12.4,
            gpu_model: Some("Apple M1".into()),
            gpu_usage_percent: Some(25.0),
            gpu_memory_used_mb: None,
            gpu_memory_total_mb: None,
            uptime_seconds: 3600,
            timestamp: 1730000000000,
        };

        let json = serde_json::to_string(&metrics).expect("serialize");
        let parsed: SystemMetrics = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed.process_memory_mb, 45.0);
        assert_eq!(parsed.gpu_model, Some("Apple M1".into()));
        assert_eq!(parsed.system_memory_percent, 50.0);
    }
}
