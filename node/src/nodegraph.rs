use eframe::egui;

fn setup_node_graph()
{
    let options = eframe::NativeOptions 
	{
		viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
		..Default::default()	
	};

    eframe::run_simple_native("subtensor visualization", options, move |ctx, frame|
    {
        egui::CentralPanel::default().show(ctx, |ui| 
        {
            ui.heading("egui");
        });
    });
}