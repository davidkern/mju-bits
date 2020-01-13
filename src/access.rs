pub trait StorageAccess { }

/// Read only access
pub struct RO;

impl StorageAccess for RO { }

/// Write only access
pub struct WO;

impl StorageAccess for WO { }

/// Read/Write access
pub struct RW;

impl StorageAccess for RW { }
