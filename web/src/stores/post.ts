import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useApi } from '@/composables/useApi'
import type { Comment, Post, PostListItem, PostStatus, VoteResult } from '@/types'

export const usePostStore = defineStore('post', () => {
  const posts = ref<PostListItem[]>([])
  const currentPost = ref<Post | null>(null)
  const comments = ref<Comment[]>([])
  const loading = ref(false)

  async function fetchPosts(boardId: string, sort?: string, status?: string) {
    const api = useApi()
    loading.value = true
    posts.value = []
    try {
      const params = new URLSearchParams()
      if (sort) params.set('sort', sort)
      if (status) params.set('status', status)
      const qs = params.toString()
      posts.value =
        (await api.get<PostListItem[]>(`/boards/${boardId}/posts${qs ? `?${qs}` : ''}`)) ?? []
    } finally {
      loading.value = false
    }
  }

  async function fetchPost(boardId: string, postId: string) {
    const api = useApi()
    loading.value = true
    try {
      currentPost.value = await api.get<Post>(`/boards/${boardId}/posts/${postId}`)
    } finally {
      loading.value = false
    }
  }

  async function fetchPostDirect(postId: string) {
    const api = useApi()
    loading.value = true
    try {
      currentPost.value = await api.get<Post>(`/posts/${postId}`)
    } finally {
      loading.value = false
    }
  }

  async function createPost(boardId: string, title: string, description?: string) {
    const api = useApi()
    const post = await api.post<Post>(`/boards/${boardId}/posts`, { title, description })
    if (!post) throw new Error('Failed to create post')
    return post
  }

  async function toggleVote(postId: string) {
    const api = useApi()
    const result = await api.post<VoteResult>(`/posts/${postId}/vote`, {})
    if (!result) throw new Error('Failed to toggle vote')

    // Update in list
    const item = posts.value.find((p) => p.id === postId)
    if (item) {
      item.has_voted = result.voted
      item.vote_count = result.vote_count
    }

    // Update current post if viewing
    if (currentPost.value?.id === postId) {
      currentPost.value.has_voted = result.voted
      currentPost.value.vote_count = result.vote_count
    }

    return result
  }

  async function fetchComments(postId: string) {
    const api = useApi()
    comments.value = (await api.get<Comment[]>(`/posts/${postId}/comments`)) ?? []
  }

  async function addComment(postId: string, body: string) {
    const api = useApi()
    const comment = await api.post<Comment>(`/posts/${postId}/comments`, { body })
    if (!comment) throw new Error('Failed to add comment')
    comments.value.push(comment)

    // Increment comment count
    if (currentPost.value?.id === postId) {
      currentPost.value.comment_count++
    }
    const listItem = posts.value.find((p) => p.id === postId)
    if (listItem) {
      listItem.comment_count++
    }

    return comment
  }

  async function deleteComment(commentId: string, postId: string) {
    const api = useApi()
    await api.del(`/comments/${commentId}`)
    comments.value = comments.value.filter((c) => c.id !== commentId)

    // Decrement comment count
    if (currentPost.value?.id === postId) {
      currentPost.value.comment_count = Math.max(0, currentPost.value.comment_count - 1)
    }
    const delItem = posts.value.find((p) => p.id === postId)
    if (delItem) {
      delItem.comment_count = Math.max(0, delItem.comment_count - 1)
    }
  }

  async function updateStatus(boardId: string, postId: string, status: PostStatus) {
    const api = useApi()
    const updated = await api.put<Post>(`/boards/${boardId}/posts/${postId}/status`, { status })
    if (!updated) throw new Error('Failed to update status')
    if (currentPost.value?.id === postId) {
      currentPost.value.status = updated.status
    }
    const statusItem = posts.value.find((p) => p.id === postId)
    if (statusItem) {
      statusItem.status = updated.status
    }
    return updated
  }

  function clear() {
    posts.value = []
    currentPost.value = null
    comments.value = []
  }

  return {
    posts,
    currentPost,
    comments,
    loading,
    fetchPosts,
    fetchPost,
    fetchPostDirect,
    createPost,
    toggleVote,
    fetchComments,
    addComment,
    deleteComment,
    updateStatus,
    clear,
  }
})
