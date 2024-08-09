## Discriminant Hash Eq 

The DiscriminantHashEq macro is designed to derive custom implementations of the Hash, PartialEq, and Eq traits for enum types.   (ONLY for enum types!) 

These implementations are based solely on the enum's variant discriminants, ignoring any associated data within the variants. 

This means that two enum values are considered equal if they are the same variant, regardless of the values of any fields/props they might contain.


### Usage 



Implement DiscriminantHashEq : 

```
#[derive(Debug, Asset, Clone, Serialize, Deserialize, DiscriminantHashEq, Reflect)]
pub enum ItemClassification {
    Consumable {
    	consumable_data: ConsumableItemData
    },
    Equipment {
        equipment_data: EquipmentData,
    },
    Gemstone {
        persistent_effects: Option<Vec<PersistentEffect>>
    },  //slots in to equipment 
    QuestItem,
    Misc 

}

```

and it will effectively be the same as adding all this: 

```

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub enum ItemClassificationType {  //discriminants only 
    Consumable,
    Equipment,
    Gemstone,
    QuestItem,
    Misc,
}


impl ItemClassification {
    pub fn get_classification_type(&self) -> ItemClassificationType {
        match self {
            ItemClassification::Consumable{..} => ItemClassificationType::Consumable,
            ItemClassification::Equipment { .. } => ItemClassificationType::Equipment,
            ItemClassification::Gemstone { .. } => ItemClassificationType::Gemstone,
            ItemClassification::QuestItem => ItemClassificationType::QuestItem,
            ItemClassification::Misc => ItemClassificationType::Misc,
        }
    }
}


impl Hash for ItemClassification {
 fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_classification_type().hash(state);
    }
}


impl PartialEq for ItemClassification {

	
  fn eq(&self, rhs: &ItemClassification) -> bool { 
  	self.get_classification_type() == rhs.get_classification_type()

  }
}
  

impl Eq for ItemClassification {}


```
