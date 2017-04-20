/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <http://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

#![allow(improper_ctypes)]

extern crate libc;

use std::ffi::{CString, CStr};
use std::{str, ptr};

use connect::{Connect, virConnectPtr};
use error::Error;

pub mod sys {
    extern crate libc;

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virDomain {
    }

    #[allow(non_camel_case_types)]
    pub type virDomainPtr = *mut virDomain;

    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[repr(C)]
    pub struct virDomainInfo {
        pub state: libc::c_ulong,
        pub maxMem: libc::c_ulong,
        pub memory: libc::c_ulong,
        pub nrVirtCpu: libc::c_uint,
        pub cpuTime: libc::c_ulong,
    }

    #[allow(non_camel_case_types)]
    pub type virDomainInfoPtr = *mut virDomainInfo;
}

#[link(name = "virt")]
extern {
    fn virDomainLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virDomainPtr;
    fn virDomainLookupByName(c: virConnectPtr, id: *const libc::c_char) -> sys::virDomainPtr;
    fn virDomainLookupByUUIDString(c: virConnectPtr, uuid: *const libc::c_char) -> sys::virDomainPtr;
    fn virDomainCreate(c: virConnectPtr) -> sys::virDomainPtr;
    fn virDomainCreateXML(c: virConnectPtr, xml: *const libc::c_char, flags: libc::c_uint) -> sys::virDomainPtr;
    fn virDomainDefineXML(c: virConnectPtr, xml: *const libc::c_char) -> sys::virDomainPtr;
    fn virDomainDefineXMLFlags(c: virConnectPtr, xml: *const libc::c_char, flags: libc::c_uint) -> sys::virDomainPtr;
    fn virDomainDestroy(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainUndefine(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainFree(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainShutdown(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainReboot(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainSuspend(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainResume(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainIsActive(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainIsUpdated(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainGetName(ptr: sys::virDomainPtr) -> *const libc::c_char;
    fn virDomainGetHostname(ptr: sys::virDomainPtr, flags: libc::c_uint) -> *const libc::c_char;
    fn virDomainGetUUIDString(ptr: sys::virDomainPtr, uuid: *mut libc::c_char) -> libc::c_int;
    fn virDomainGetXMLDesc(ptr: sys::virDomainPtr, flags: libc::c_uint) -> *const libc::c_char;
    fn virDomainGetAutostart(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainSetAutostart(ptr: sys::virDomainPtr, autostart: libc::c_uint) -> libc::c_int;
    fn virDomainGetID(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainSetMaxMemory(ptr: sys::virDomainPtr, memory: libc::c_ulong) -> libc::c_int;
    fn virDomainSetMemory(ptr: sys::virDomainPtr, memory: libc::c_ulong) -> libc::c_int;
    fn virDomainSetMemoryFlags(ptr: sys::virDomainPtr, memory: libc::c_ulong, flags: libc::c_uint) -> libc::c_int;
    fn virDomainSetMemoryStatsPeriod(ptr: sys::virDomainPtr, period: libc::c_int, flags: libc::c_uint) -> libc::c_int;
    fn virDomainSetVcpus(ptr: sys::virDomainPtr, vcpus: libc::c_uint) -> libc::c_int;
    fn virDomainSetVcpusFlags(ptr: sys::virDomainPtr, vcpus: libc::c_uint, flags: libc::c_uint) -> libc::c_int;
    fn virDomainGetVcpusFlags(ptr: sys::virDomainPtr, vcpus: libc::c_uint) -> libc::c_int;
    fn virDomainRestore(c: virConnectPtr, source: *const libc::c_char) -> libc::c_int;
    fn virDomainRestoreFlags(c: virConnectPtr, source: *const libc::c_char, flags: libc::c_uint) -> libc::c_int;
    fn virDomainGetConnect(ptr: sys::virDomainPtr) -> virConnectPtr;
    fn virDomainGetInfo(ptr: sys::virDomainPtr, ninfo: sys::virDomainInfoPtr) -> libc::c_int;
}

pub type DomainXMLFlags = self::libc::c_uint;
pub const VIR_DOMAIN_XML_SECURE: DomainXMLFlags = 1 << 0;
pub const VIR_DOMAIN_XML_INACTIVE: DomainXMLFlags = 1 << 1;
pub const VIR_DOMAIN_XML_UPDATE_CPU: DomainXMLFlags = 1 << 2;
pub const VIR_DOMAIN_XML_MIGRATABLE: DomainXMLFlags = 1 << 3;

pub type DomainCreateFlags = self::libc::c_uint;
pub const VIR_DOMAIN_NONE: DomainCreateFlags = 0;
pub const VIR_DOMAIN_START_PAUSED: DomainCreateFlags = 1 << 0;
pub const VIR_DOMAIN_START_AUTODESTROY: DomainCreateFlags = 1 << 1;
pub const VIR_DOMAIN_START_BYPASS_CACHE: DomainCreateFlags = 1 << 2;
pub const VIR_DOMAIN_START_FORCE_BOOT: DomainCreateFlags = 1 << 3;
pub const VIR_DOMAIN_START_VALIDATE: DomainCreateFlags = 1 << 4;

pub type DomainModImpactFlags = self::libc::c_uint;
pub const VIR_DOMAIN_AFFECT_CURRENT: DomainModImpactFlags = 0;
pub const VIR_DOMAIN_AFFECT_LIVE: DomainModImpactFlags = 1 << 0;
pub const VIR_DOMAIN_AFFECT_CONFIG: DomainModImpactFlags = 1 << 1;

pub type DomainMemoryModFlags = self::libc::c_uint;
pub const VIR_DOMAIN_MEM_CURRENT: DomainMemoryModFlags = VIR_DOMAIN_AFFECT_CURRENT;
pub const VIR_DOMAIN_MEM_LIVE: DomainMemoryModFlags = VIR_DOMAIN_AFFECT_LIVE;
pub const VIR_DOMAIN_MEM_CONFIG: DomainMemoryModFlags = VIR_DOMAIN_AFFECT_CONFIG;
pub const VIR_DOMAIN_MEM_MAXIMUM: DomainMemoryModFlags = 1 << 2;

pub type DomainVcpuFlags = self::libc::c_uint;
pub const VIR_DOMAIN_VCPU_CURRENT: DomainVcpuFlags = VIR_DOMAIN_AFFECT_CURRENT;
pub const VIR_DOMAIN_VCPU_LIVE: DomainVcpuFlags = VIR_DOMAIN_AFFECT_LIVE;
pub const VIR_DOMAIN_VCPU_CONFIG: DomainVcpuFlags = VIR_DOMAIN_AFFECT_CONFIG;
pub const VIR_DOMAIN_VCPU_MAXIMUM: DomainVcpuFlags = 1 << 2;
pub const VIR_DOMAIN_VCPU_GUEST: DomainVcpuFlags = 1 << 3;
pub const VIR_DOMAIN_VCPU_HOTPLUGGABLE: DomainVcpuFlags = 1 << 4;

pub type DomainDefineFlags = self::libc::c_uint;
pub const VIR_DOMAIN_DEFINE_VALIDATE: DomainDefineFlags = 1 << 0;

pub type DomainSaveRestoreFlags = self::libc::c_uint;
pub const VIR_DOMAIN_SAVE_BYPASS_CACHE: DomainSaveRestoreFlags = 1 << 0;
pub const VIR_DOMAIN_SAVE_RUNNING: DomainSaveRestoreFlags = 1 << 1;
pub const VIR_DOMAIN_SAVE_PAUSED: DomainSaveRestoreFlags = 1 << 2;

pub type DomainState = self::libc::c_uint;
pub const VIR_DOMAIN_NOSTATE: DomainState = 0;
pub const VIR_DOMAIN_RUNNING: DomainState = 1;
pub const VIR_DOMAIN_BLOCKED: DomainState = 2;
pub const VIR_DOMAIN_PAUSED: DomainState  = 3;
pub const VIR_DOMAIN_SHUTDOWN: DomainState = 4;
pub const VIR_DOMAIN_SHUTOFF: DomainState = 5;
pub const VIR_DOMAIN_CRASHED: DomainState = 6;
pub const VIR_DOMAIN_PMSUSPENDED: DomainState = 7;

pub struct DomainInfo {
    pub state: DomainState,
    pub max_mem: u64,
    pub memory: u64,
    pub nr_virt_cpu: u32,
    pub cpu_time: u64,
}

pub struct Domain {
    ptr: sys::virDomainPtr
}

impl Drop for Domain {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            if self.free().is_err() {
                panic!("Unable to drop memory for Domain")
            }
            return;
        }
    }
}

impl Domain {

    pub fn new(ptr: sys::virDomainPtr) -> Domain {
        return Domain{ptr: ptr}
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virDomainGetConnect(self.ptr);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByName(
                conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByUUIDString(
                conn.as_ptr(), CString::new(uuid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetName(self.ptr);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
        }
    }

    pub fn get_hostname(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetHostname(self.ptr, flags as libc::c_uint);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virDomainGetUUIDString(self.ptr, uuid.as_mut_ptr()) == -1 {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(
                uuid.as_ptr()).to_string_lossy().into_owned())
        }
    }

    pub fn get_id(&self) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainGetID(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_xml_desc(&self, flags:DomainCreateFlags) -> Result<String, Error> {
        unsafe {
            let xml = virDomainGetXMLDesc(self.ptr, flags);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(xml).to_string_lossy().into_owned())
        }
    }

    pub fn create(conn: &Connect) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainCreate(conn.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn get_info(&self) -> Result<DomainInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainInfo{
                state: 0,
                maxMem: 0,
                memory: 0,
                nrVirtCpu: 0,
                cpuTime: 0,
            };
            let res = virDomainGetInfo(self.ptr, pinfo);
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(DomainInfo{
                state: (*pinfo).state as DomainState,
                max_mem: (*pinfo).maxMem as u64,
                memory: (*pinfo).memory as u64,
                nr_virt_cpu: (*pinfo).nrVirtCpu as u32,
                cpu_time: (*pinfo).cpuTime as u64,
            })
        }
    }

    pub fn create_xml(conn: &Connect, xml: &str, flags: DomainCreateFlags) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainCreateXML(
                conn.as_ptr(),  CString::new(xml).unwrap().as_ptr(),
                flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }


    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainDefineXML(
                conn.as_ptr(),  CString::new(xml).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn define_xml_flags(conn: &Connect, xml: &str,
                      flags: DomainDefineFlags) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainDefineXMLFlags(
                conn.as_ptr(), CString::new(xml).unwrap().as_ptr(), flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virDomainDestroy(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn shutdown(&self) -> Result<(), Error> {
        unsafe {
            if virDomainShutdown(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn reboot(&self) -> Result<(), Error> {
        unsafe {
            if virDomainReboot(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn suspend(&self) -> Result<(), Error> {
        unsafe {
            if virDomainSuspend(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn resume(&self) -> Result<(), Error> {
        unsafe {
            if virDomainResume(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainIsActive(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virDomainUndefine(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virDomainFree(self.ptr) == -1 {
                return Err(Error::new());
            }
            self.ptr = ptr::null_mut();
            return Ok(());
        }
    }

    pub fn is_updated(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainIsUpdated(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainGetAutostart(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetAutostart(self.ptr, autostart as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_max_memory(&self, memory: u64) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMaxMemory(self.ptr, memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_memory(&self, memory: u64) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMemory(self.ptr, memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_memory_flags(&self, memory: u64,
                                 flags: DomainMemoryModFlags) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMemoryFlags(self.ptr,
                                              memory as libc::c_ulong,
                                              flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_memory_stats_period(&self, period: i32,
                                   flags: DomainMemoryModFlags) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMemoryStatsPeriod(self.ptr,
                                                    period as libc::c_int,
                                                    flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_vcpus(&self, vcpus: u32) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetVcpus(self.ptr, vcpus as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_vcpus_flags(&self, vcpus: u32,
                                flags: DomainVcpuFlags) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetVcpusFlags(self.ptr,
                                             vcpus as libc::c_uint,
                                             flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn domain_restore(conn: &Connect, path: &str) -> Result<(), Error> {
        unsafe {
            if virDomainRestore(
                conn.as_ptr(), CString::new(path).unwrap().as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn domain_restore_flags(conn: &Connect, path: &str, flags: DomainSaveRestoreFlags) -> Result<(), Error> {
        unsafe {
            if virDomainRestoreFlags(
                conn.as_ptr(), CString::new(path).unwrap().as_ptr(),
                flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn get_vcpus_flags(&self, flags: DomainVcpuFlags) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainGetVcpusFlags(self.ptr, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }
}
