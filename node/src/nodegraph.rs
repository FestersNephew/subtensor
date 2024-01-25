use eframe::egui;

include!("nodegraph_emission.rs");
include!("nodegraph_stake.rs");

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
                    "time: {:?} block_reward: {:?}", 
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    pallet_subtensor::get_block_emission()
                )
            );
            
            static mut option: i32 = 0; 
            ui.horizontal(|ui|
            {
                ui.selectable_value(&mut option, 0, "emission flow"); 
                ui.selectable_value(&mut option, 1, "stake");
            });
        
            match option
            {
                0 => paint_nodegraph_emission(ui),
                1 => paint_nodegraph_stake(ui),
                _ => {}
            }
        });

        ctx.request_repaint();
    });
}
