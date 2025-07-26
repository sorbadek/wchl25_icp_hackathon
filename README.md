# Peerverse

Peerverse is a decentralized platform built on the Internet Computer Protocol (ICP) where anyone can teach, learn, and exchange knowledge. It is designed to create a balanced, value-driven learning economy owned and sustained by its users.

## What is Peerverse?
Peerverse enables users to:
- **Earn points (XP)** by sharing knowledge, teaching, helping, or contributing insights.
- **Spend points** to learn from others. If you only consume, your balance runs out, so you must contribute or pay to continue learning.
- **Participate in a transparent, trustless, and fair ecosystem** governed by smart contracts, with no intermediaries.

### Key Features
- **Structured Courses:** Created by individuals or institutions.
- **Knowledge Marketplace:** Trade resources and learning materials.
- **Freelancer Discovery Hub:** Connect skilled users with opportunities.
- **Community Spaces:** Shared learning and collaboration.
- **Private Learning Ecosystems:** Organizations can build internal Peerverse instances for staff, with HR tracking participation and performance.
- **Tamper-proof Certificates:** Certificates are securely tied to users via blockchain-based storage.

Peerverse is not just another EdTech platform—it's a new learning economy powered by the belief that everyone has something to teach.

---

## Project Structure

- `src/user_profile.rs`: User registration, profile management, skills, certificates.
- `src/content.rs`: Courses, resources, marketplace listings.
- `src/session.rs`: Learning sessions, progress, and interactions.
- `src/credential.rs`: Certificate issuance and verification.
- `src/xp.rs`: XP (points) logic.
- `src/lib.rs`: Exposes all canister methods for deployment.
- `src/video_session.rs`: Video sessions (like X Spaces) where users can host, join, and earn XP for live participation.

---

## Local Development Setup

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install-dfx)
- [Node.js](https://nodejs.org/) (optional, for frontend integration)

### Steps

1. **Clone the repository:**
   ```sh
   git clone <your-repo-url>
   cd wchl25_icp_hackathon
   ```

2. **Install DFX:**
   Follow the official ICP documentation to install DFX.

3. **Start the local ICP replica:**
   ```sh
   dfx start --background
   ```

4. **Deploy the canister:**
   ```sh
   dfx deploy
   ```

5. **Interact with the backend:**
   Use DFX commands or the Candid UI to call canister methods (e.g., mint XP, create sessions, register users).

6. **(Optional) Integrate with a frontend:**
   You can build a frontend using React, Vue, or any framework and connect to the canister using ICP agent libraries.

---

## Example Canister Methods
- `mint_xp(user_id, amount)`: Add XP to a user.
- `burn_xp(user_id, amount)`: Remove XP from a user.
- `get_xp(user_id)`: Query a user's XP balance.
- `register_user(user_id, name, bio)`: Register a new user.
- `create_course(id, title, description, creator)`: Create a new course.
- `create_session(owner, title, description, required_xp)`: Create a learning session.
- `issue_certificate(id, user_id, course_id, issued_at)`: Issue a certificate.
- `create_video_session(host, title, description, xp_reward)`: Create a new video session.
- `join_video_session(session_id, user_id)`: Join a video session.
- `end_video_session(session_id)`: End a video session.
- `reward_xp_for_video_session(session_id)`: Reward XP to host and participants after session ends.
- `get_video_session(session_id)`: Query details of a video session.

---

## Contributing
Pull requests and suggestions are welcome! Please open issues for bugs or feature requests.

---

## License
This project is open source and available under the MIT License.
