struct StealthUpdater {
    current_model: Arc,
    staging_model: Mutex>,
    update_signal: AtomicBool,
}

impl StealthUpdater {
    fn update_model(&self, new_model: DetectionModel) {
        // 1. 后台加载新模型
        *self.staging_model.lock() = Some(new_model);
        
        // 2. 原子切换（无服务中断）
        self.update_signal.store(true, Ordering::Release);
        
        // 3. 模型差分：只传输变化部分
        let diff = compute_model_diff(&self.current_model, &new_model);
        broadcast_update(diff);
    }
    
    fn check_update(&self) {
        if self.update_signal.load(Ordering::Acquire) {
            if let Some(new_model) = self.staging_model.lock().take() {
                // 原子替换
                self.current_model = Arc::new(new_model);
                self.update_signal.store(false, Ordering::Release);
            }
        }
    }
}