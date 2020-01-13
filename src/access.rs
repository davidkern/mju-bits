pub trait StorageAccess { }
pub trait StorageReadAccess: StorageAccess { }
pub trait StorageWriteAccess: StorageAccess { }

/// Read only access
pub struct RO;

impl StorageAccess for RO { }
impl StorageReadAccess for RO { }

/// Write only access
pub struct WO;

impl StorageAccess for WO { }
impl StorageWriteAccess for WO { }

/// Read/Write access
pub struct RW;

impl StorageAccess for RW { }
impl StorageReadAccess for RW { }
impl StorageWriteAccess for RW { }
