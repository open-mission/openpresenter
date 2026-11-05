use op_core::SystemMetrics;
use std::time::Instant;
use sysinfo::{
    CpuRefreshKind, Gpus, MemoryRefreshKind, Pid, ProcessRefreshKind, ProcessesToUpdate,
    RefreshKind, System,
};

/// Coletor de métricas de telemetria de hardware e processo.
pub struct MetricsCollector {
    sys: System,
    gpus: Option<Gpus>,
    pid: Pid,
    start_time: Instant,
    last_refresh: Instant,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    /// Inicializa o coletor de métricas com a instância do sistema e lista de GPUs.
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        )
        .or_else(|_| System::new_all())
        .expect("Failed to initialize sysinfo System");

        let gpus = Gpus::new_with_refreshed_list().ok();
        let pid = Pid::from_u32(std::process::id());
        let now = Instant::now();

        Self {
            sys,
            gpus,
            pid,
            start_time: now,
            last_refresh: now,
        }
    }

    /// Executa a leitura atualizada de CPU, RAM, GPU e processo.
    pub fn collect(&mut self) -> SystemMetrics {
        let now = Instant::now();

        // 1. Atualiza CPU global e memória RAM do sistema
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();

        // 2. Atualiza CPU e memória física (RSS) do processo da aplicação
        self.sys.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[self.pid]),
            true,
            ProcessRefreshKind::nothing().with_cpu().with_memory(),
        );

        // 3. Atualiza métricas de GPU se disponível
        if let Some(ref mut gpus) = self.gpus {
            gpus.refresh(false);
        }

        self.last_refresh = now;

        // Extrai dados do processo
        let (proc_cpu, proc_mem_bytes) = if let Some(p) = self.sys.process(self.pid) {
            (p.cpu_usage(), p.memory())
        } else {
            (0.0, 0)
        };
        let proc_mem_mb = (proc_mem_bytes as f64) / (1024.0 * 1024.0);

        // Extrai dados do sistema
        let sys_mem_total_bytes = self.sys.total_memory();
        let sys_mem_used_bytes = self.sys.used_memory();
        let sys_mem_total_mb = (sys_mem_total_bytes as f64) / (1024.0 * 1024.0);
        let sys_mem_used_mb = (sys_mem_used_bytes as f64) / (1024.0 * 1024.0);
        let sys_mem_percent = if sys_mem_total_bytes > 0 {
            ((sys_mem_used_bytes as f32) / (sys_mem_total_bytes as f32)) * 100.0
        } else {
            0.0
        };
        let sys_cpu_percent = self.sys.global_cpu_usage();

        // Extrai dados da GPU
        let (gpu_model, gpu_usage_percent, gpu_mem_used_mb, gpu_mem_total_mb) =
            if let Some(ref gpus) = self.gpus {
                if let Some(first) = gpus.first() {
                    let model = first
                        .model()
                        .or_else(|| first.vendor())
                        .map(|s| s.to_string());
                    let usage = first.usage();
                    let used_mb = first.used_memory().map(|b| (b as f64) / (1024.0 * 1024.0));
                    let total_mb = first.total_memory().map(|b| (b as f64) / (1024.0 * 1024.0));
                    (model, usage, used_mb, total_mb)
                } else {
                    (None, None, None, None)
                }
            } else {
                (None, None, None, None)
            };

        let uptime_seconds = self.start_time.elapsed().as_secs();
        let timestamp = chrono::Utc::now().timestamp_millis();

        SystemMetrics {
            process_memory_bytes: proc_mem_bytes,
            process_memory_mb: (proc_mem_mb * 10.0).round() / 10.0,
            process_cpu_percent: (proc_cpu * 10.0).round() / 10.0,
            system_memory_total_bytes: sys_mem_total_bytes,
            system_memory_used_bytes: sys_mem_used_bytes,
            system_memory_total_mb: (sys_mem_total_mb * 10.0).round() / 10.0,
            system_memory_used_mb: (sys_mem_used_mb * 10.0).round() / 10.0,
            system_memory_percent: (sys_mem_percent * 10.0).round() / 10.0,
            system_cpu_percent: (sys_cpu_percent * 10.0).round() / 10.0,
            gpu_model,
            gpu_usage_percent: gpu_usage_percent.map(|u| (u * 10.0).round() / 10.0),
            gpu_memory_used_mb: gpu_mem_used_mb.map(|m| (m * 10.0).round() / 10.0),
            gpu_memory_total_mb: gpu_mem_total_mb.map(|m| (m * 10.0).round() / 10.0),
            uptime_seconds,
            timestamp,
        }
    }
}
