use window::Window;
use model::ListModel;

pub struct Controller<'a> {
    window: &'a mut Window,
    model: &'a ListModel<'a>
}

impl<'a> Controller<'a> {
    pub fn new(window: &'a mut Window, model: &'a ListModel) -> Controller<'a> {
        Controller {
            window: window,
            model: model
        }
    }

    /*************************
     * CALLBACK
     ************************/

    pub fn on_init(&self) {
        self.window.draw(self.model);
    }

    pub fn on_next_active_sub(&mut self){
        if !self.model.subscriptions.is_empty() {
            if self.window.active_sub + 1 < self.model.subscriptions.len() as i32 {
                self.window.active_sub += 1;
                self.window.draw(self.model);
            }
        }
    }

    pub fn on_previous_active_sub(&mut self){
        if !self.model.subscriptions.is_empty() {
            if self.window.active_sub - 1 >= 0 {
                self.window.active_sub -= 1;
                self.window.draw(self.model);
            }
        }
    }

}