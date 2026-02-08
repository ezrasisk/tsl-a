use crate::quaternion::Quaternion;

pub struct GaugeMetrics {
    pub norm: f64,
    pub angle_deg: f64,
    pub current_signal: String,
    pub lagrangian: f64,
    pub predicted_q: Quaternion,
    pub predicted_angle: f64,
    pub speculative_signal: String,
}

pub fn compute_gauge_and_prediction(
    raw_q: Quaternion,
    volume: f64,
    prev_unit_q: Quaternion,
) -> GaugeMetrics {
    let norm = raw_q.norm();
    let unit_q = raw_q.normalized();
    let angle_deg = unit_q.rotation_angle_deg();

    let alignment = unit_q.w.abs();
    let direction = if unit_q.w > 0.0 { "BUY" } else { "SELL" };

    let current_signal = if norm > 15.0 && alignment > 0.7 {
        format!("STRONG {} (Low Curvature)", direction)
    } else if norm > 15.0 && alignment > 0.4 {
        format!("{} (Moderate Curvature)", direction)
    } else if alignment < 0.3 {
        "HOLD - High Curvature".to_string()
    } else {
        "HOLD - Neutral".to_string()
    };

    // Lagrangian terms
    let dt = 1.0;
    let velocity = unit_q.add(&prev_unit_q.scale(-1.0));
    let kinetic = 0.5 * velocity.norm().powi(2);

    let k_align = 2.0;
    let k_curve = 1.0;
    let k_vol = 0.1;

    let v_align = -k_align * unit_q.w.powi(2);
    let curvature_proxy = angle_deg / 180.0;
    let v_curve = k_curve * curvature_proxy.powi(2);
    let v_vol = -k_vol * volume.ln();

    let lagrangian = kinetic + v_align - v_curve + v_vol;

    // Speculative prediction: gradient step toward higher Lagrangian
    let step_size = 0.3;
    let grad_align = Quaternion::new(2.0 * k_align * unit_q.w, 0.0, 0.0, 0.0);
    let predicted_q = unit_q.add(&grad_align.scale(step_size)).normalized();

    let predicted_angle = predicted_q.rotation_angle_deg();
    let pred_direction = if predicted_q.w > 0.0 { "BUY" } else { "SELL" };
    let speculative_signal = if predicted_angle < 60.0 {
        format!("STRONG {} (Predicted Low Curvature)", pred_direction)
    } else {
        format!("{} (Predicted Evolving Curvature)", pred_direction)
    };

    GaugeMetrics {
        norm,
        angle_deg,
        current_signal,
        lagrangian,
        predicted_q,
        predicted_angle,
        speculative_signal,
    }
}
