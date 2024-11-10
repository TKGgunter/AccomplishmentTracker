use crate::Deserialize;
use crate::{EventType, LeadershipPrinciples, TomlEvent, TomlAccomplishmentData};

#[warn(dead_code)]
#[derive(Deserialize, Debug)]
struct Event{
    summary: String,
    details: Option<String>,
    date:    toml::value::Datetime,
 
    customer_obsession:   Option<u8>,
    ownership:            Option<u8>,
    invent_and_simplify:  Option<u8>,
    are_right_alot:       Option<u8>,
    learn_and_be_curious: Option<u8>,
    hire_and_develop_the_best: Option<u8>,
    insist_on_the_highest_standards: Option<u8>,
    think_big:          Option<u8>,
    bias_for_action:    Option<u8>,
    frugality:          Option<u8>,
    earn_trust:         Option<u8>,
    dive_deep:          Option<u8>,
    have_backbone:      Option<u8>,
    deliver_results:    Option<u8>,
    strive_best_employer: Option<u8>,
    success_and_scale_brings_responsibility:  Option<u8>,
 
    tags: Option<String>,
}

#[derive(Deserialize)]
pub struct Report{
    events: Vec<Event>,
}

impl Report {
    pub fn write_in_new_format(self) -> String {
        let mut td = TomlAccomplishmentData{
            events: vec![]
        };

        for it in self.events {
            td.events.push(old_to_new_event(&it));
        }

        toml::to_string(&td).expect("Could not create toml string.")
    }
}

fn old_to_new_event(event: &Event) -> TomlEvent {
    let mut lp = vec![];
    {
        if event.customer_obsession.is_some() {
            lp.push(LeadershipPrinciples::CustomerObsession);
        }

        if event.ownership.is_some() {
            lp.push(LeadershipPrinciples::Ownership);
        }

        if event.invent_and_simplify.is_some() {
            lp.push(LeadershipPrinciples::InventAndSimplify);
        }

        if event.are_right_alot.is_some() {
            lp.push(LeadershipPrinciples::AreRightALot);
        }

        if event.learn_and_be_curious.is_some() {
            lp.push(LeadershipPrinciples::LearnAndBeCurious);
        }

        if event.hire_and_develop_the_best.is_some() {
            lp.push(LeadershipPrinciples::HireAndDevelopTheBest);
        }

        if event.insist_on_the_highest_standards.is_some() {
            lp.push(LeadershipPrinciples::InsistOnTheHighestStandards);
        }

        if event.think_big.is_some() {
            lp.push(LeadershipPrinciples::ThinkBig);
        }

        if event.bias_for_action.is_some() {
            lp.push(LeadershipPrinciples::BiasForAction);
        }

        if event.frugality.is_some() {
            lp.push(LeadershipPrinciples::Frugality);
        }

        if event.earn_trust.is_some() {
            lp.push(LeadershipPrinciples::EarnTrust);
        }

        if event.dive_deep.is_some() {
            lp.push(LeadershipPrinciples::DiveDeep);
        }

        if event.have_backbone.is_some() {
            lp.push(LeadershipPrinciples::HaveBackboneAndCommit);
        }

        if event.deliver_results.is_some() {
            lp.push(LeadershipPrinciples::DeliverResults);
        }

        // if event.strive_best_employer.is_some() {
        //     lp.push(LeadershipPrinciples::);
        // }

        if event.success_and_scale_brings_responsibility.is_some() {
            lp.push(LeadershipPrinciples::SuccessAndScaleBringBroadResponsibility);
        }
    }

    let event_type = match event.tags.as_ref() {
        Some(s) => {
            match s.to_lowercase().as_str() { 
                "bar raiser" => {
                    EventType::BarRaise
                },
                "invest in your self" => {
                    EventType::InvestInYourSelf
                }
                _ => {
                    println!("unexpected entry: {}", s);
                    EventType::None
                }
            }
        },
        None =>{
            EventType::None
        }
    };

    TomlEvent{
        date: event.date,
        leadership_principles: lp,
        event_type,
        summary: event.summary.clone(),
        details: event.details.as_ref().unwrap().clone(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_v1_to_v2_report() {
        let old = r#"
[[events]]
summary = "This is an example."
date = 1979-05-27T07:32:00-08:00
details = """ 
This document was created using a proprietary tool found here, https://github.com/TKGgunter/AZTracker. <b>Testing</b>.
asdf
### Header 3
 
*Wild boy*
"""
ownership = 1
earn_trust = 1
dive_deep = 1
"#;


        let old_toml: Report = toml::from_str(old).expect("Old report could not be deserialzied.");
        let new_toml = old_toml.write_in_new_format();


        let expected_new_toml_string = r#"[[events]]
date = 1979-05-27T07:32:00-08:00
leadership_principles = ["Ownership", "EarnTrust", "DiveDeep"] 
event_type = "None"
summary = "This is an example."
details = """ 

This document was created using a proprietary tool found here, https://github.com/TKGgunter/AZTracker. <b>Testing</b>.
asdf
### Header 3
 
*Wild boy*
""""#;

        // Not the best test.
        assert!(new_toml.len() == expected_new_toml_string.len());

    }
}
