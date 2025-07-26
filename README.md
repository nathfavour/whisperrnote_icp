# whisperrnote_icp Backend Structure

This backend is structured to mirror the application and data model of the original [whisperrnote](https://github.com/buildathonzx/whisperrnote) project. Each module in `backend/src` corresponds to a major entity in the appwrite data model (users, notes, tags, etc.).

## Structure

- `src/users/` - User management logic
- `src/notes/` - Notes CRUD and logic
- `src/tags/` - Tagging system
- `src/api_keys/` - API key management
- `src/blog_posts/` - Blog post logic
- `src/comments/` - Comments on notes and posts
- `src/extensions/` - Extensions/plugins
- `src/reactions/` - Reactions (emoji, etc.)
- `src/collaborators/` - Note collaborators
- `src/activity_log/` - Activity logging
- `src/settings/` - User/app settings

This structure is designed for easy integration with the main whisperrnote frontend and to maintain compatibility with the appwrite data model.

For more details, see the original repo: https://github.com/buildathonzx/whisperrnote
