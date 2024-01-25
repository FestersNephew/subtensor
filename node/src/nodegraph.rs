use eframe::egui;

fn setup_node_graph()
{
    let options = eframe::NativeOptions 
	{
		viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
		..Default::default()	
	};

    eframe::run_simple_native("subtensor visualization", options, move |ctx, frame|
    {
        egui::CentralPanel::default().show(ctx, |ui| 
        {
            ui.heading(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_string());

            ctx.request_repaint();
        });
    });
}
