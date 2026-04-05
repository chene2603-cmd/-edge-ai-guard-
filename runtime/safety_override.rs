// 快速接管机制：模型降级
struct SafetyOverride {
    primary_model: Box,      // 主模型（可能欺骗）
    backup_model: TinyModel,           // 可信小模型
    switch_controller: AtomicBool,     // 切换控制
}

impl SafetyOverride {
    fn on_deception_detected(&self, detection: DetectionResult) -> SafeOutput {
        // 立即中断主模型生成
        self.switch_controller.store(true, Ordering::SeqCst);
        
        // 无缝切换到可信模型
        let safe_output = if detection.score > CRITICAL_THRESHOLD {
            // 紧急模式：规则引擎
            self.rule_engine.generate_safe_response()
        } else if detection.score > WARNING_THRESHOLD {
            // 降级模式：小模型接管
            self.backup_model.continue_generation()
        } else {
            // 监控模式：标记但继续
            MarkedOutput(self.primary_model.output())
        };
        
        // 记录审计日志
        self.audit_log.record_incident(detection, safe_output.clone());
        
        safe_output
    }
}