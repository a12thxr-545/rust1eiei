import { Injectable, signal, PLATFORM_ID, inject } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';

export type LanguageType = 'en' | 'th';

interface Translations {
    [key: string]: string;
}

const EN_TRANSLATIONS: Translations = {
    // Navbar
    'nav.missions': 'Missions',
    'nav.create': 'Create',
    'nav.profile': 'Profile',
    'nav.logout': 'Logout',
    'nav.login': 'Login',

    // Home
    'home.hero.title': 'Connect & Conquer',
    'home.hero.subtitle': 'Join forces with fighters worldwide. Create missions, build your crew, and achieve greatness together.',
    'home.hero.explore': 'Explore Missions',
    'home.hero.create': 'Create Mission',
    'home.features.title': 'Why Join Us?',
    'home.features.missions': 'Epic Missions',
    'home.features.missions.desc': 'Create and join missions with fighters around the world',
    'home.features.crew': 'Build Your Crew',
    'home.features.crew.desc': 'Connect with like-minded brawlers and form alliances',
    'home.features.compete': 'Compete & Win',
    'home.features.compete.desc': 'Track your progress and climb the leaderboards',
    'home.latest': 'Latest Missions',
    'home.viewAll': 'View all',
    'home.cta.title': 'Ready to Start?',
    'home.cta.subtitle': 'Join thousands of fighters on epic missions',
    'home.cta.button': 'Get Started',

    // Missions
    'missions.title': 'Missions',
    'missions.subtitle': 'Explore and manage your space missions',
    'missions.explore': 'Explore',
    'missions.participating': 'Participating',
    'missions.myMissions': 'My Missions',
    'missions.inbox': 'Inbox',
    'missions.search': 'Search by Room ID (e.g. AB123)...',
    'missions.loading': 'Loading missions...',
    'missions.noMissions': 'No missions from others yet',
    'missions.noMissionsDesc': 'Wait for other users to create missions!',
    'missions.roomId': 'Room ID',
    'missions.mission': 'Mission',
    'missions.status': 'Status',
    'missions.crew': 'Crew',
    'missions.created': 'Created',
    'missions.leader': 'Leader',

    // Actions
    'action.view': 'View',
    'action.join': 'Join',
    'action.leave': 'Leave',
    'action.edit': 'Edit',
    'action.delete': 'Delete',
    'action.kick': 'Kick',
    'action.invite': 'Invite',
    'action.addFriend': 'Add Friend',
    'action.accept': 'Accept',
    'action.reject': 'Reject',
    'action.send': 'Send',
    'action.cancel': 'Cancel',
    'action.save': 'Save Changes',
    'action.create': 'Create Mission',
    'action.start': 'Start Mission',
    'action.success': 'Success',
    'action.fail': 'Fail',

    // Modal
    'modal.details': 'Mission Details',
    'modal.edit': 'Edit Mission',
    'modal.delete': 'Delete Mission',
    'modal.create': 'Create New Mission',
    'modal.deleteConfirm': 'Are you sure you want to delete',
    'modal.deleteWarning': 'This action cannot be undone.',
    'modal.name': 'Mission Name',
    'modal.description': 'Description',
    'modal.descOptional': 'Description (Optional)',
    'modal.crewMembers': 'Crew Members',
    'modal.inviteFriends': 'Invite Friends',
    'modal.chat': 'Mission Canal',
    'modal.noMessages': 'No messages yet. Lead the conversation!',
    'modal.chatLocked': 'Join this mission to chat with the crew.',
    'modal.typePlaceholder': 'Type a message...',

    // Inbox
    'inbox.friends': 'My Brawlers Friends',
    'inbox.searchBrawlers': 'Search Brawlers',
    'inbox.searchPlaceholder': 'Find fighters to connect with...',
    'inbox.friendRequests': 'Friend Requests',
    'inbox.missionInvitations': 'Mission Invitations',
    'inbox.noFriends': 'No friends yet. Search for brawlers below to grow your squad!',
    'inbox.friend': 'Friend',

    // Profile
    'profile.editCover': 'Edit Cover',
    'profile.completed': 'Completed',
    'profile.joined': 'Joined',

    // Login
    'login.title': 'Welcome Back',
    'login.subtitle': 'Sign in to continue your journey',
    'login.email': 'Email or Username',
    'login.password': 'Password',
    'login.button': 'Sign In',
    'login.noAccount': "Don't have an account?",
    'login.register': 'Register',
    'login.demo': 'Demo credentials',

    // Members
    'members.title': 'Brawlers',
    'members.subtitle': 'Search and connect with other fighters',
    'members.search': 'Search brawlers by name or username...',
    'members.noResults': 'No brawlers found',
    'members.tryAgain': 'Try searching with a different name',
};

const TH_TRANSLATIONS: Translations = {
    // Navbar
    'nav.missions': 'ภารกิจ',
    'nav.create': 'สร้าง',
    'nav.profile': 'โปรไฟล์',
    'nav.logout': 'ออกจากระบบ',
    'nav.login': 'เข้าสู่ระบบ',

    // Home
    'home.hero.title': 'รวมพล & พิชิต',
    'home.hero.subtitle': 'รวมพลังกับนักสู้ทั่วโลก สร้างภารกิจ รวมทีม และประสบความสำเร็จไปด้วยกัน',
    'home.hero.explore': 'สำรวจภารกิจ',
    'home.hero.create': 'สร้างภารกิจ',
    'home.features.title': 'ทำไมต้องร่วมทีม?',
    'home.features.missions': 'ภารกิจสุดมันส์',
    'home.features.missions.desc': 'สร้างและเข้าร่วมภารกิจกับนักสู้ทั่วโลก',
    'home.features.crew': 'สร้างทีมของคุณ',
    'home.features.crew.desc': 'เชื่อมต่อกับนักสู้ที่มีแนวคิดเดียวกันและสร้างพันธมิตร',
    'home.features.compete': 'แข่งขัน & ชนะ',
    'home.features.compete.desc': 'ติดตามความก้าวหน้าและไต่อันดับลีดเดอร์บอร์ด',
    'home.latest': 'ภารกิจล่าสุด',
    'home.viewAll': 'ดูทั้งหมด',
    'home.cta.title': 'พร้อมเริ่มต้นหรือยัง?',
    'home.cta.subtitle': 'ร่วมกับนักสู้หลายพันคนในภารกิจสุดมันส์',
    'home.cta.button': 'เริ่มเลย',

    // Missions
    'missions.title': 'ภารกิจ',
    'missions.subtitle': 'สำรวจและจัดการภารกิจอวกาศของคุณ',
    'missions.explore': 'สำรวจ',
    'missions.participating': 'กำลังร่วม',
    'missions.myMissions': 'ภารกิจของฉัน',
    'missions.inbox': 'กล่องข้อความ',
    'missions.search': 'ค้นหาด้วยรหัสห้อง (เช่น AB123)...',
    'missions.loading': 'กำลังโหลดภารกิจ...',
    'missions.noMissions': 'ยังไม่มีภารกิจจากคนอื่น',
    'missions.noMissionsDesc': 'รอให้ผู้ใช้คนอื่นสร้างภารกิจ!',
    'missions.roomId': 'รหัสห้อง',
    'missions.mission': 'ภารกิจ',
    'missions.status': 'สถานะ',
    'missions.crew': 'ลูกทีม',
    'missions.created': 'สร้างเมื่อ',
    'missions.leader': 'หัวหน้า',

    // Actions
    'action.view': 'ดู',
    'action.join': 'เข้าร่วม',
    'action.leave': 'ออก',
    'action.edit': 'แก้ไข',
    'action.delete': 'ลบ',
    'action.kick': 'เตะออก',
    'action.invite': 'เชิญ',
    'action.addFriend': 'เพิ่มเพื่อน',
    'action.accept': 'ยอมรับ',
    'action.reject': 'ปฏิเสธ',
    'action.send': 'ส่ง',
    'action.cancel': 'ยกเลิก',
    'action.save': 'บันทึก',
    'action.create': 'สร้างภารกิจ',
    'action.start': 'เริ่มภารกิจ',
    'action.success': 'สำเร็จ',
    'action.fail': 'ล้มเหลว',

    // Modal
    'modal.details': 'รายละเอียดภารกิจ',
    'modal.edit': 'แก้ไขภารกิจ',
    'modal.delete': 'ลบภารกิจ',
    'modal.create': 'สร้างภารกิจใหม่',
    'modal.deleteConfirm': 'คุณแน่ใจหรือไม่ว่าต้องการลบ',
    'modal.deleteWarning': 'การกระทำนี้ไม่สามารถย้อนกลับได้',
    'modal.name': 'ชื่อภารกิจ',
    'modal.description': 'คำอธิบาย',
    'modal.descOptional': 'คำอธิบาย (ไม่บังคับ)',
    'modal.crewMembers': 'สมาชิกทีม',
    'modal.inviteFriends': 'เชิญเพื่อน',
    'modal.chat': 'ห้องแชทภารกิจ',
    'modal.noMessages': 'ยังไม่มีข้อความ เป็นคนแรกที่พูดคุย!',
    'modal.chatLocked': 'เข้าร่วมภารกิจนี้เพื่อแชทกับทีม',
    'modal.typePlaceholder': 'พิมพ์ข้อความ...',

    // Inbox
    'inbox.friends': 'เพื่อนนักสู้ของฉัน',
    'inbox.searchBrawlers': 'ค้นหานักสู้',
    'inbox.searchPlaceholder': 'ค้นหานักสู้เพื่อเชื่อมต่อ...',
    'inbox.friendRequests': 'คำขอเป็นเพื่อน',
    'inbox.missionInvitations': 'คำเชิญเข้าภารกิจ',
    'inbox.noFriends': 'ยังไม่มีเพื่อน ค้นหานักสู้ด้านล่างเพื่อขยายทีม!',
    'inbox.friend': 'เพื่อน',

    // Profile
    'profile.editCover': 'แก้ไขหน้าปก',
    'profile.completed': 'สำเร็จแล้ว',
    'profile.joined': 'เข้าร่วมแล้ว',

    // Login
    'login.title': 'ยินดีต้อนรับกลับ',
    'login.subtitle': 'เข้าสู่ระบบเพื่อดำเนินการต่อ',
    'login.email': 'อีเมลหรือชื่อผู้ใช้',
    'login.password': 'รหัสผ่าน',
    'login.button': 'เข้าสู่ระบบ',
    'login.noAccount': 'ยังไม่มีบัญชี?',
    'login.register': 'ลงทะเบียน',
    'login.demo': 'ข้อมูลทดสอบ',

    // Members
    'members.title': 'นักสู้',
    'members.subtitle': 'ค้นหาและเชื่อมต่อกับนักสู้คนอื่น',
    'members.search': 'ค้นหานักสู้ด้วยชื่อหรือ username...',
    'members.noResults': 'ไม่พบนักสู้',
    'members.tryAgain': 'ลองค้นหาด้วยชื่ออื่น',
};

@Injectable({
    providedIn: 'root'
})
export class LanguageService {
    private platformId = inject(PLATFORM_ID);
    language = signal<LanguageType>('en');

    private translations: Record<LanguageType, Translations> = {
        'en': EN_TRANSLATIONS,
        'th': TH_TRANSLATIONS
    };

    constructor() {
        if (isPlatformBrowser(this.platformId)) {
            const savedLang = localStorage.getItem('language') as LanguageType;
            if (savedLang && ['en', 'th'].includes(savedLang)) {
                this.language.set(savedLang);
            }
        }
    }

    toggleLanguage() {
        const newLang: LanguageType = this.language() === 'en' ? 'th' : 'en';
        this.setLanguage(newLang);
    }

    setLanguage(lang: LanguageType) {
        this.language.set(lang);
        if (isPlatformBrowser(this.platformId)) {
            localStorage.setItem('language', lang);
        }
    }

    t(key: string): string {
        const lang = this.language();
        return this.translations[lang][key] || this.translations['en'][key] || key;
    }
}
