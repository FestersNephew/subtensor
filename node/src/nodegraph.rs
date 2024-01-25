use eframe::egui;

unsafe fn setup_node_graph()
{
    let options = eframe::NativeOptions 
	{
		viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
		..Default::default()	
	};

    let _ = eframe::run_simple_native("subtensor visualization", options, move |ctx, frame|
    {
        egui::CentralPanel::default().show(ctx, |ui| 
        {
            ui.label(
                format!(
                    "time {:?} block_reward: {:?}", 
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    1_000_000_000
                )
            );
            
            static mut option: i32 = 0; 
            ui.selectable_value(&mut option, 0, "emission flow"); 
            ui.selectable_value(&mut option, 1, "stake");

            match option
            {
                0 => 
                {

                },

                1 => 
                {
                    
                },
                
                _ => {}
            }
        });
    });
}
